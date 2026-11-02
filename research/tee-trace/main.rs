#[cfg(all(target_os = "linux", target_arch = "arm"))]
mod live;
mod headers;
mod model;
mod wire;
fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() < 4 || args[0] != "--log" || args[2] != "--" {
        eprintln!("usage: tee-trace --log NEW_LOG.jsonl -- /absolute/ARM32/program [args...]\nLaunches under ptrace from startup; no late attach. Metadata only. Fatal tracer errors kill the traced tree. Job-control group stops are unsupported.");
        std::process::exit(2)
    }
    #[cfg(all(target_os = "linux", target_arch = "arm"))]
    match live::run(std::path::Path::new(&args[1]), &args[3..]) {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("tee-trace: {e}");
            std::process::exit(125)
        }
    }
    #[cfg(not(all(target_os = "linux", target_arch = "arm")))]
    {
        eprintln!("tee-trace requires Linux ARM32; host builds support offline tests only");
        std::process::exit(2)
    }
}
