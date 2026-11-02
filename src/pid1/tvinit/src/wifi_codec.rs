//! Owned native inputs for the Lean Wi-Fi policy, called only on Core's executor.
use super::lean_runtime::{self, Object};
use std::{cell::Cell, io, sync::OnceLock};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Network {
    pub ssid: String,
    pub signal: String,
    pub secure: bool,
}

extern "C" {
    fn initialize_tv_TV_Wifi(builtin: u8) -> Object;
    fn tv_wifi_scan(text: Object) -> Object;
    fn tv_wifi_configuration(ssid: Object, password: Object, secure: u32) -> Object;
    fn tv_wifi_io_unit(unused: u32) -> Object;
    fn lean_mk_string_from_bytes(data: *const u8, length: usize) -> Object;
}
static INITIALIZED: OnceLock<Result<(), String>> = OnceLock::new();
#[derive(Default)]
struct Output {
    networks: Vec<Network>,
    configuration: Option<io::Result<String>>,
}
thread_local! { static CURRENT: Cell<*mut Output> = const { Cell::new(std::ptr::null_mut()) }; }
struct Scope;
impl Drop for Scope {
    fn drop(&mut self) { CURRENT.with(|p| p.set(std::ptr::null_mut())); }
}
fn capture(action: impl FnOnce() -> Object) -> io::Result<Output> {
    let _executor = lean_runtime::ThreadGuard::attach();
    match INITIALIZED.get_or_init(|| unsafe {
        lean_runtime::consume_io_unit(initialize_tv_TV_Wifi(1)).map_err(|e| e.to_string())
    }) {
        Ok(()) => (),
        Err(message) => return Err(io::Error::other(message.clone())),
    }
    let mut output = Output::default();
    CURRENT.with(|p| {
        assert!(p.get().is_null(), "recursive Wi-Fi policy call");
        p.set(&mut output);
    });
    let scope = Scope;
    let result = unsafe { lean_runtime::consume_io_unit(action()) };
    drop(scope);
    result?;
    Ok(output)
}
fn input(text: &str) -> Object {
    unsafe { lean_mk_string_from_bytes(text.as_ptr(), text.len()) }
}
#[repr(C)]
struct LeanString { header: [u8; 8], size: usize, capacity: usize, length: usize }
/// `object` is a borrowed valid String for the duration of its Lean callback.
unsafe fn copied_string(object: Object) -> String {
    let string = &*object.cast::<LeanString>();
    debug_assert_eq!(string.header[7], 249);
    let bytes = std::slice::from_raw_parts(
        object.cast::<u8>().add(std::mem::size_of::<LeanString>()), string.size - 1);
    std::str::from_utf8_unchecked(bytes).to_owned()
}
fn output(action: impl FnOnce(&mut Output)) {
    CURRENT.with(|p| {
        assert!(!p.get().is_null(), "Wi-Fi callback outside policy call");
        unsafe { action(&mut *p.get()) }
    });
}
#[no_mangle]
unsafe extern "C" fn tv_wifi_network(ssid: Object, signal: Object, secure: u32) -> Object {
    output(|capture| capture.networks.push(Network {
        ssid: copied_string(ssid), signal: copied_string(signal), secure: secure != 0,
    }));
    tv_wifi_io_unit(0)
}
#[no_mangle]
unsafe extern "C" fn tv_wifi_result(success: u32, text: Object) -> Object {
    output(|capture| {
        let text = copied_string(text);
        capture.configuration = Some(if success == 1 { Ok(text) } else { Err(io::Error::other(text)) });
    });
    tv_wifi_io_unit(0)
}

pub fn parse_scan(text: &str) -> io::Result<Vec<Network>> {
    Ok(capture(|| unsafe { tv_wifi_scan(input(text)) })?.networks)
}
pub fn configuration(ssid: &str, password: &str, secure: bool) -> io::Result<String> {
    capture(|| unsafe { tv_wifi_configuration(input(ssid), input(password), u32::from(secure)) })?
        .configuration.ok_or_else(|| io::Error::other("Wi-Fi policy did not return a configuration"))?
}
