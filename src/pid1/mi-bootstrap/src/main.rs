use mi_bootstrap::{Device, Transport};
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.is_empty() || args == ["--help"] {
        println!(
            "mi-bootstrap status | amp-status | amp-read-status (read-only; initialization is available only to the owning PID 1)"
        );
        return Ok(());
    }
    if args == ["amp-status"] {
        println!("{:?}", mi_bootstrap::amplifier::snapshot()?);
        return Ok(());
    }
    if args == ["amp-read-status"] {
        println!("{:?}", mi_bootstrap::amplifier::read_control_status()?);
        return Ok(());
    }
    if args != ["status"] {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "probe is read-only; INIT requires audited PID-1 cold-boot provenance",
        ));
    }
    let mut device = Device::open()?;
    for s in [40, 0, 3, 1, 6, 7, 9, 17, 18, 19] {
        println!("selector={s} metadata_ready={}", device.status(s)?);
    }
    Ok(())
}
