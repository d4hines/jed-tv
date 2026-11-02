#[path = "../../../src/pid1/audio/aout-route.rs"]
mod aout_route;
// Standalone test listener for the PID1-owned direct backend; no hardware initialization.
#[path = "direct-backend.rs"]
mod direct_backend;
#[path = "../../../src/pid1/audio/mi-pcm.rs"]
mod mi_pcm;
#[path = "pcm-converter.rs"]
mod pcm_converter;
#[path = "../../../src/pid1/audio/protocol.rs"]
mod protocol;
use std::io;
fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let mut port = 8879;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--port" => {
                port = args
                    .next()
                    .and_then(|n| n.parse().ok())
                    .ok_or_else(|| io::Error::other("--port NUMBER"))?
            }
            "--help" => {
                println!("Usage: direct-audio-bridge [--port PORT]\nTemporary single-owner MI PCM test listener; hardware must already be initialized and other owners quiesced.");
                return Ok(());
            }
            _ => return Err(io::Error::other("unknown option")),
        }
    }
    if !std::fs::read_to_string("/proc/cmdline")?.contains("androidboot.serialno=4C0490290FA05A0C0")
    {
        return Err(io::Error::other("requires original G06"));
    }
    let backend = direct_backend::DirectBackend::with_speaker_route(aout_route::RoutePolicy::RequireEmpty);
    protocol::serve(backend, port)
}
