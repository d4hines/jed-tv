#[cfg(all(target_os = "linux", target_arch = "arm"))]
fn run() -> std::io::Result<()> {
    use g06_tee_provider::{
        linux::Device,
        pins::{PinnedStore, ROOT},
        serve_one,
    };
    use g06_tee_rpc::{linux::LinuxAllocator, Limits, Provider};
    g06_tee_provider::linux::check_identity()?;
    let store = PinnedStore::preload(std::path::Path::new(ROOT))?;
    let mut transport = Device::open()?;
    eprintln!(
        "g06-tee-provider: pinned4 TAs; exclusive privileged OP-TEE {:?}",
        transport.version
    );
    let limits = Limits::default();
    let allocator = LinuxAllocator::new(transport.allocator_file()?, limits.region_bytes);
    let mut provider = Provider::new(allocator, store, limits).map_err(std::io::Error::other)?;
    let mut count = 0u64;
    loop {
        let result = serve_one(&mut transport, &mut provider)?;
        count += 1;
        if count <= 10000 {
            eprintln!(
                "g06-tee-provider: request={count} function={} meta={} args={} status={:#010x}",
                result.function, result.meta_count, result.logical_count, result.status
            )
        } else if count == 10001 {
            eprintln!("g06-tee-provider: further per-request logs suppressed")
        }
    }
}
fn main() {
    if std::env::args_os().len() != 1 {
        eprintln!(
            "usage: g06-tee-provider-candidate (fixed G06 device and four pinned stock TAs)"
        );
        std::process::exit(2)
    }
    #[cfg(all(target_os = "linux", target_arch = "arm"))]
    if let Err(e) = run() {
        eprintln!("g06-tee-provider: {e}");
        std::process::exit(1)
    }
    #[cfg(not(all(target_os = "linux", target_arch = "arm")))]
    {
        eprintln!("g06-tee-provider is an ARM32 Linux candidate; host build supports tests only");
        std::process::exit(2)
    }
}
