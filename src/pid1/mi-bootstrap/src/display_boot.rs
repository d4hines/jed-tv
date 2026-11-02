//! Exact controller-open and boot-logo handoff for the exclusive cold owner.
use super::{Device, Module, Transport};
use std::{collections::BTreeSet, io, os::fd::AsRawFd};

const GET_CONTROLLER: u32 = 0xc0081206;
const OPEN_CONTROLLER: u32 = 0xc0081204;
const GET_OUTPUT_TIMING: u32 = 0xc008122d;
const OSD_DISABLE_LOGO: u32 = 0x144e;
const DISP_DISABLE_LOGO: u32 = 0xc0041203;
const CONTROLLER: u32 = 0x36004354;

#[derive(Debug, Eq, PartialEq)]
pub struct DisplayHandoff {
    /// Held by PID 1 for the rest of this boot. There is no automatic close.
    pub controller: u32,
    /// Exact MI_DISP_Timing_e readback: 6=1080i50 or 12=1080p60.
    /// Both use the evidenced 1920x1080 OSD destination coordinate space.
    pub timing: u32,
}

fn authorize(completed: &BTreeSet<Module>, attempted: &mut bool) -> io::Result<()> {
    if *attempted {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "display handoff was already attempted; no retry in this boot",
        ));
    }
    if super::cold_start::DISPLAY_SEQUENCE
        .iter()
        .any(|m| !completed.contains(m))
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "this cold owner must complete DISP, DISPOUT and OSD INIT first",
        ));
    }
    *attempted = true;
    Ok(())
}

trait Io {
    fn call(&mut self, module: Module, request: u32, words: &mut [u32]) -> io::Result<i32>;
}
struct Kernel<'a>(&'a Device);
impl Io for Kernel<'_> {
    fn call(&mut self, module: Module, request: u32, words: &mut [u32]) -> io::Result<i32> {
        let file = self.0.modules.get(&module).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "initialized module descriptor missing",
            )
        })?;
        if request == OSD_DISABLE_LOGO {
            // This no-argument legacy request does not take the MI envelope.
            let code =
                unsafe { super::ioctl(file.as_raw_fd(), request as _, std::ptr::null_mut::<u8>()) };
            if code < 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(code)
            }
        } else {
            super::call(
                file,
                request,
                words.as_mut_ptr() as u64,
                std::mem::size_of_val(words) as u16,
            )
        }
    }
}

fn sequence(io: &mut impl Io) -> io::Result<DisplayHandoff> {
    let check = |code: i32, operation: &str| {
        if code == 0 {
            Ok(())
        } else {
            Err(io::Error::other(format!(
                "{operation}: vendor result {code}; handoff stopped"
            )))
        }
    };
    let mut words = [0, 0];
    let absent = io.call(Module::Display, GET_CONTROLLER, &mut words)?;
    if absent != 3 || words[0] != 0 {
        return Err(io::Error::other(format!(
            "cold controller must be absent before open: vendor={absent}, handle={:#x}",
            words[0]
        )));
    }
    words = [0, 0];
    check(
        io.call(Module::Display, OPEN_CONTROLLER, &mut words)?,
        "DISP OpenController",
    )?;
    if words[0] != CONTROLLER {
        return Err(io::Error::other(
            "unexpected V109 controller handle after open",
        ));
    }
    words = [0, 0];
    check(
        io.call(Module::Display, GET_CONTROLLER, &mut words)?,
        "DISP GetController",
    )?;
    if words[0] != CONTROLLER {
        return Err(io::Error::other(
            "controller readback does not match opened handle",
        ));
    }
    let mut timing = [CONTROLLER, u32::MAX];
    check(
        io.call(Module::Display, GET_OUTPUT_TIMING, &mut timing)?,
        "DISP GetOutputTiming",
    )?;
    // Exact firmware enum definition: E_MI_DISP_TIMING_1920X1080_50I=6,
    // E_MI_DISP_TIMING_1920X1080_60P=12. The former is the cold INI
    // default; the latter is the physically tested warm mode. Do not accept
    // other dimensions/modes before constructing the fixed OSD geometry.
    if timing[0] != CONTROLLER || !matches!(timing[1], 6 | 12) {
        return Err(io::Error::other(format!(
            "unsupported controller output timing {} (handle {:#x}); boot logo retained",
            timing[1], timing[0]
        )));
    }
    check(
        io.call(Module::Osd, OSD_DISABLE_LOGO, &mut [])?,
        "OSD DisableBootLogo",
    )?;
    check(
        io.call(Module::Display, DISP_DISABLE_LOGO, &mut [CONTROLLER])?,
        "DISP DisableBootLogo",
    )?;
    Ok(DisplayHandoff {
        controller: CONTROLLER,
        timing: timing[1],
    })
}

pub(super) fn run(device: &mut Device) -> io::Result<DisplayHandoff> {
    device.ensure_exclusive_cold_boot()?;
    if device.status(1)? {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "SYS_USER became ready before the owned cold display handoff",
        ));
    }
    authorize(&device.completed, &mut device.display_handoff_attempted)?;
    sequence(&mut Kernel(device))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fake {
        calls: Vec<(u32, Vec<u32>)>,
        fail: Option<u32>,
        preexisting: bool,
        bad_readback: bool,
        timing: u32,
    }
    impl Fake {
        fn new() -> Self {
            Self {
                calls: vec![],
                fail: None,
                preexisting: false,
                bad_readback: false,
                timing: 12,
            }
        }
    }
    impl Io for Fake {
        fn call(&mut self, _: Module, request: u32, words: &mut [u32]) -> io::Result<i32> {
            self.calls.push((request, words.to_vec()));
            if self.fail == Some(request) {
                return Ok(7);
            }
            if request == GET_CONTROLLER && self.calls.len() == 1 && !self.preexisting {
                return Ok(3);
            }
            if request == GET_OUTPUT_TIMING {
                words[1] = self.timing;
            }
            if request == GET_CONTROLLER || request == OPEN_CONTROLLER {
                words[0] = if self.bad_readback && request == GET_CONTROLLER {
                    0
                } else {
                    CONTROLLER
                };
            }
            Ok(0)
        }
    }
    #[test]
    fn opens_once_then_reads_back_and_hands_off_in_proven_order() {
        let mut io = Fake::new();
        let handoff = sequence(&mut io).unwrap();
        assert_eq!(handoff.controller, CONTROLLER);
        assert_eq!(handoff.timing, 12);
        assert_eq!(
            io.calls,
            vec![
                (GET_CONTROLLER, vec![0, 0]),
                (OPEN_CONTROLLER, vec![0, 0]),
                (GET_CONTROLLER, vec![0, 0]),
                (GET_OUTPUT_TIMING, vec![CONTROLLER, u32::MAX]),
                (OSD_DISABLE_LOGO, vec![]),
                (DISP_DISABLE_LOGO, vec![CONTROLLER])
            ]
        );
    }
    #[test]
    fn existing_controller_never_opens_or_disables_logo() {
        let mut io = Fake::new();
        io.preexisting = true;
        assert!(sequence(&mut io).is_err());
        assert_eq!(io.calls.len(), 1);
    }
    #[test]
    fn open_failure_and_bad_readback_never_disable_logo() {
        let mut io = Fake::new();
        io.fail = Some(OPEN_CONTROLLER);
        assert!(sequence(&mut io).is_err());
        assert_eq!(io.calls.len(), 2);
        let mut io = Fake::new();
        io.bad_readback = true;
        assert!(sequence(&mut io).is_err());
        assert_eq!(io.calls.len(), 3);
    }
    #[test]
    fn osd_handoff_failure_prevents_disp_handoff() {
        let mut io = Fake::new();
        io.fail = Some(OSD_DISABLE_LOGO);
        assert!(sequence(&mut io).is_err());
        assert_eq!(io.calls.len(), 5);
    }
    #[test]
    fn evidenced_cold_and_warm_timings_are_returned() {
        for timing in [6, 12] {
            let mut io = Fake::new();
            io.timing = timing;
            assert_eq!(sequence(&mut io).unwrap().timing, timing);
        }
    }
    #[test]
    fn unsupported_or_failed_timing_never_removes_boot_logo() {
        for timing in [0, 5, 7, 11, 13, u32::MAX] {
            let mut io = Fake::new();
            io.timing = timing;
            assert!(sequence(&mut io).is_err());
            assert_eq!(io.calls.len(), 4);
            assert_eq!(io.calls.last().unwrap().0, GET_OUTPUT_TIMING);
        }
        let mut io = Fake::new();
        io.fail = Some(GET_OUTPUT_TIMING);
        assert!(sequence(&mut io).is_err());
        assert_eq!(io.calls.len(), 4);
    }
    #[test]
    fn incomplete_init_and_repeated_attempt_are_refused() {
        let mut completed = BTreeSet::new();
        let mut attempted = false;
        assert!(authorize(&completed, &mut attempted).is_err());
        assert!(!attempted);
        completed.extend(super::super::cold_start::DISPLAY_SEQUENCE.iter().copied());
        authorize(&completed, &mut attempted).unwrap();
        assert!(authorize(&completed, &mut attempted).is_err());
    }
}
