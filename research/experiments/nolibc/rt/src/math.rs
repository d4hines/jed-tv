//! libm entry points std's float methods call (f32::powf -> powf, ...),
//! from the `libm` crate (musl's libm ported to Rust).

#[no_mangle]
pub extern "C" fn powf(x: f32, y: f32) -> f32 {
    libm::powf(x, y)
}

#[no_mangle]
pub extern "C" fn pow(x: f64, y: f64) -> f64 {
    libm::pow(x, y)
}
