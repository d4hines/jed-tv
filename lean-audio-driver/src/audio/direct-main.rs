//! The Lean audio driver as its own process, for a TV whose audio is not owned
//! by our PID1 (the G13 trial: stock init stays PID 1, Android's audioserver
//! and the audio HAL are stopped first). Same driver, same mechanisms as PID1
//! on the G06; only the TV identity and the listening port come from the
//! command line. Static, no Android libraries: raw MI ioctls only.
use std::{io, net::TcpListener, sync::Arc, time::Instant};
use tvinit::lean_runtime::{discard_io_result, Object};

extern "C" {
    /// Lake's package-prefixed initializer for TV/Audio/Native.lean; it also
    /// initializes the Lean runtime itself.
    fn initialize_tv_TV_Audio_Native(builtin: u8) -> Object;
    fn lean_io_mark_end_initialization();
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let mut port = 8877u16;
    let mut serial: Option<String> = None;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--port" => port = args.next().and_then(|v| v.parse().ok()).ok_or_else(|| io::Error::other("--port NUMBER"))?,
            "--serial" => serial = args.next(),
            "--help" => {
                println!("Usage: direct-audio --serial TV_SERIAL [--port PORT]\n\
                          Lean direct MI PCM driver. Hardware must already be initialized (warm) and every\n\
                          other media owner stopped; refuses to run on any other TV than --serial.");
                return Ok(());
            }
            _ => return Err(io::Error::other(format!("unknown option {argument}"))),
        }
    }
    let serial = serial.ok_or_else(|| io::Error::other("--serial is required: the TV this build was checked for"))?;
    mi_audio::service::verify_ownership(&serial)?;
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    let start = Instant::now();
    let log: mi_audio::service::Log = Arc::new(move |message: &str| {
        println!("[{:.3}] {message}", start.elapsed().as_secs_f64());
    });
    unsafe {
        discard_io_result(initialize_tv_TV_Audio_Native(1))?;
        lean_io_mark_end_initialization();
    }
    log(&format!("direct-audio: TV {serial}, media owners stopped, Lean driver initialized"));
    mi_audio::service::serve(listener, log);
    Ok(())
}
