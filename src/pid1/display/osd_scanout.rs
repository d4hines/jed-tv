//! PID1-owned external OSD surfaces. No fbdev open or SYS readiness publication.
use crate::{mma::UnpublishedBuffer, osd_abi};
use std::{ffi::c_void, fs::File, io, os::fd::AsRawFd};

#[repr(C)]
struct Envelope {
    bytes: u16,
    reserved: [u8; 6],
    pointer: u64,
}
const _: () = assert!(std::mem::size_of::<Envelope>() == 16);
extern "C" {
    fn ioctl(fd: i32, request: u32, argument: *mut c_void) -> i32;
}

trait Transport {
    fn call(&mut self, request: u32, words: &mut [u32]) -> io::Result<()>;
}
struct Osd(File);
impl Transport for Osd {
    fn call(&mut self, request: u32, words: &mut [u32]) -> io::Result<()> {
        let mut envelope = Envelope {
            bytes: std::mem::size_of_val(words) as u16,
            reserved: [0; 6],
            pointer: words.as_mut_ptr() as usize as u64,
        };
        let result = unsafe {
            ioctl(
                self.0.as_raw_fd(),
                request,
                (&mut envelope as *mut Envelope).cast(),
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::other(format!(
                "OSD ioctl {request:#x}: vendor={result}, errno={}",
                io::Error::last_os_error()
            )))
        }
    }
}

struct Effects<'a> {
    transport: &'a mut dyn Transport,
    errors: [Option<io::Error>; 2],
}
impl Effects<'_> {
    fn finish(mut self, result: u32) -> io::Result<u32> {
        match (self.errors[0].take(), self.errors[1].take()) {
            (Some(first), Some(second)) => Err(io::Error::other(format!(
                "{first}; closing configuration: {second}"
            ))),
            (Some(error), _) | (_, Some(error)) => Err(error),
            _ if result >> 8 != 0 => Err(io::Error::other("invalid Lean scanout state")),
            _ => Ok(result),
        }
    }
}

// Lean calls this only synchronously, using the live exclusive Effects pointer
// supplied below. The effect does not retain it or enter the Zr allocator.
#[no_mangle]
extern "C" fn tv_display_osd_call(
    context: usize,
    request: u32,
    first: u32,
    second: u32,
    count: u8,
) -> u8 {
    let effects = unsafe { &mut *(context as *mut Effects<'_>) };
    let result = match count {
        1 => effects.transport.call(request, &mut [first]),
        2 => effects.transport.call(request, &mut [first, second]),
        _ => Err(io::Error::other("invalid Lean OSD argument count")),
    };
    match result {
        Ok(()) => 0,
        Err(error) => {
            let slot = if effects.errors[0].is_none() { 0 } else { 1 };
            effects.errors[slot] = Some(error);
            1
        }
    }
}

extern "C" {
    fn tv_scanout_present(context: usize, layer: u32, window: u32, surface: u32, state: u32)
        -> u32;
    fn tv_scanout_hide(context: usize, layer: u32, state: u32) -> u32;
    fn tv_scanout_configure_alpha(context: usize, layer: u32, window: u32) -> u32;
}

fn with_effects(io: &mut impl Transport, run: impl FnOnce(usize) -> u32) -> io::Result<u32> {
    let mut effects = Effects {
        transport: io,
        errors: [None, None],
    };
    let result = run((&mut effects as *mut Effects<'_>) as usize);
    effects.finish(result)
}

fn present(
    io: &mut impl Transport,
    layer: u32,
    window: u32,
    surface: u32,
    state: u32,
) -> io::Result<u32> {
    with_effects(io, |context| unsafe {
        tv_scanout_present(context, layer, window, surface, state)
    })
}

fn hide(io: &mut impl Transport, layer: u32, state: u32) -> io::Result<u32> {
    with_effects(io, |context| unsafe {
        tv_scanout_hide(context, layer, state)
    })
}

fn configure_alpha(io: &mut impl Transport, layer: u32, window: u32) -> io::Result<()> {
    with_effects(io, |context| unsafe {
        tv_scanout_configure_alpha(context, layer, window)
    })
    .map(|_| ())
}

pub struct Scanout {
    osd: Osd,
    memory: Option<UnpublishedBuffer>,
    layer: u32,
    window: u32,
    surfaces: [u32; 2],
    state: u32,
    published: bool,
}
impl Scanout {
    pub fn create() -> io::Result<Self> {
        if std::process::id() != 1
            || !std::fs::read_to_string("/proc/cmdline")?
                .split_whitespace()
                .any(|word| word == "androidboot.serialno=4C0490290FA05A0C0")
        {
            return Err(io::Error::other(
                "external scanout requires original G06 PID1",
            ));
        }
        let file = File::open("/dev/mik/osd").or_else(|_| File::open("/dev/mik!osd"))?;
        let osd = Osd(file);
        // Unlike ordinary call(), this query accepts the absent-layer response.
        // Any other result must not create over an owner.
        let mut absent = [0u32, 1];
        let mut envelope = Envelope {
            bytes: 8,
            reserved: [0; 6],
            pointer: absent.as_mut_ptr() as usize as u64,
        };
        let result = unsafe {
            ioctl(
                osd.0.as_raw_fd(),
                0xc0081456,
                (&mut envelope as *mut Envelope).cast(),
            )
        };
        if result != 3 || absent[0] != 0 {
            return Err(io::Error::other(format!(
                "OSD layer1 must be absent: vendor={result}, handle={:#x}",
                absent[0]
            )));
        }
        let memory = UnpublishedBuffer::allocate()?;
        let mut scanout = Self {
            osd,
            memory: Some(memory),
            layer: 0,
            window: 0,
            surfaces: [0; 2],
            state: 0,
            published: false,
        };
        let mut layer = osd_abi::layer(1).map_err(io::Error::other)?;
        scanout.osd.call(osd_abi::LAYER_CREATE, &mut layer)?;
        if layer[0] != 0x20001 {
            return Err(io::Error::other("unexpected OSD layer1 handle"));
        }
        scanout.layer = layer[0];
        for page in 0..2 {
            let mut surface = osd_abi::surface(scanout.memory.as_ref().unwrap().address(), page)
                .map_err(io::Error::other)?;
            // Even a failed syscall can have published metadata before returning.
            // Keep backing memory for PID1's lifetime from this point onward.
            scanout.published = true;
            scanout.osd.call(osd_abi::SURFACE_CREATE, &mut surface)?;
            if surface[0] & 0xffff0000 != 0x80000 {
                return Err(io::Error::other("invalid external OSD surface handle"));
            }
            scanout.surfaces[page as usize] = surface[0];
        }
        if scanout.surfaces[0] == scanout.surfaces[1] {
            return Err(io::Error::other("OSD returned duplicate surface handles"));
        }
        let mut window = osd_abi::window(scanout.layer).map_err(io::Error::other)?;
        scanout.osd.call(osd_abi::WINDOW_CREATE, &mut window)?;
        if window[0] & 0xffff0000 != 0x40000 {
            return Err(io::Error::other("invalid external OSD window handle"));
        }
        scanout.window = window[0];
        configure_alpha(&mut scanout.osd, scanout.layer, scanout.window)?;
        eprintln!(
            "OSD external layer={:#x} window={:#x} surfaces={:x?} IOVA={:#x}; no fbdev",
            scanout.layer,
            scanout.window,
            scanout.surfaces,
            scanout.memory.as_ref().unwrap().address()
        );
        Ok(scanout)
    }

    pub fn draw_rgba(&mut self, pixels: &[u32]) -> io::Result<()> {
        self.memory
            .as_mut()
            .unwrap()
            .write_rgba_page(self.state & 1, pixels)?;
        self.state = present(
            &mut self.osd,
            self.layer,
            self.window,
            self.surfaces[(self.state & 1) as usize],
            self.state,
        )?;
        Ok(())
    }
    pub fn hide(&mut self) -> io::Result<()> {
        self.state = hide(&mut self.osd, self.layer, self.state)?;
        Ok(())
    }
}
impl Drop for Scanout {
    fn drop(&mut self) {
        if self.layer != 0 {
            let _ = self.osd.call(0xc0041406, &mut [self.layer]);
        }
        if self.published {
            // V109 cannot destroy windows reliably and takes no DMA-BUF ref for
            // PHY_OS. Leaking the descriptor is deliberate and bounded: PID1
            // retains it until reboot. This class cannot run in a short-lived app.
            if let Some(memory) = self.memory.take() {
                std::mem::forget(memory);
            }
            eprintln!("OSD external backing retained until reboot; do not retry creation");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fake {
        calls: Vec<u32>,
        fail: Vec<u32>,
        arguments: Vec<Vec<u32>>,
    }
    impl Transport for Fake {
        fn call(&mut self, request: u32, words: &mut [u32]) -> io::Result<()> {
            self.calls.push(request);
            self.arguments.push(words.to_vec());
            if self.fail.contains(&request) {
                Err(io::Error::other(format!("failed {request:#x}")))
            } else {
                Ok(())
            }
        }
    }
    #[test]
    fn flip_is_bracketed_by_begin_and_apply() {
        let mut fake = Fake {
            calls: vec![],
            fail: vec![],
            arguments: vec![],
        };
        assert_eq!(present(&mut fake, 0x20001, 0x40001, 0x80001, 2).unwrap(), 3);
        assert_eq!(fake.calls, [0xc0041416, 0xc008142d, 0xc0041417]);
    }
    #[test]
    fn failed_flip_still_applies_and_reports_both_errors() {
        let mut fake = Fake {
            calls: vec![],
            fail: vec![0xc008142d, 0xc0041417],
            arguments: vec![],
        };
        let error = present(&mut fake, 1, 2, 3, 2).unwrap_err().to_string();
        assert_eq!(fake.calls, [0xc0041416, 0xc008142d, 0xc0041417]);
        assert!(error.contains("0xc008142d") && error.contains("0xc0041417"));
    }
    #[test]
    fn failed_begin_does_not_flip() {
        let mut fake = Fake {
            calls: vec![],
            fail: vec![0xc0041416],
            arguments: vec![],
        };
        assert!(present(&mut fake, 1, 2, 3, 2).is_err());
        assert_eq!(fake.calls, [0xc0041416]);
    }
    #[test]
    fn configures_all_three_fbdev_alpha_controls() {
        let mut fake = Fake {
            calls: vec![],
            fail: vec![],
            arguments: vec![],
        };
        configure_alpha(&mut fake, 0x20001, 0x40001).unwrap();
        assert_eq!(fake.calls, [0xc004140c, 0xc0081425, 0xc0041415]);
        assert_eq!(
            fake.arguments,
            [vec![0x20001], vec![0x40001, 0xff01], vec![0x20001]]
        );
    }
    #[test]
    fn all_present_faults_preserve_state_and_close_successful_begin() {
        const BEGIN: u32 = 0xc0041416;
        const FLIP: u32 = 0xc008142d;
        const APPLY: u32 = 0xc0041417;
        const SHOW: u32 = 0xc0041405;
        for state in 0..4 {
            for failures in 0..16 {
                let fail: Vec<_> = [BEGIN, FLIP, APPLY, SHOW]
                    .into_iter()
                    .enumerate()
                    .filter_map(|(bit, call)| (failures & (1 << bit) != 0).then_some(call))
                    .collect();
                let mut fake = Fake {
                    calls: vec![],
                    fail,
                    arguments: vec![],
                };
                let result = present(&mut fake, 0x20001, 0x40002, 0x80003, state);
                let mut expected = vec![BEGIN];
                let mut success = false;
                if failures & 1 == 0 {
                    expected.extend([FLIP, APPLY]);
                    if failures & 6 == 0 {
                        if state & 2 == 0 {
                            expected.push(SHOW);
                        }
                        success = state & 2 != 0 || failures & 8 == 0;
                    }
                }
                assert_eq!(fake.calls, expected, "state={state}, fail={failures}");
                assert_eq!(result.is_ok(), success, "state={state}, fail={failures}");
                if let Ok(next) = result {
                    assert_eq!(next, (state ^ 1) | 2);
                }
                assert_eq!(fake.arguments[0], [0x20001]);
                if failures & 1 == 0 {
                    assert_eq!(fake.arguments[1], [0x40002, 0x80003]);
                    assert_eq!(fake.arguments[2], [0x20001]);
                }
            }
        }
    }
    #[test]
    fn hide_state_changes_only_after_success() {
        for state in 0..4 {
            for failing in [false, true] {
                let mut fake = Fake {
                    calls: vec![],
                    fail: if failing { vec![0xc0041406] } else { vec![] },
                    arguments: vec![],
                };
                let result = hide(&mut fake, 0x20001, state);
                if state & 2 == 0 {
                    assert_eq!(result.unwrap(), state);
                    assert!(fake.calls.is_empty());
                } else {
                    assert_eq!(fake.calls, [0xc0041406]);
                    assert_eq!(fake.arguments, [vec![0x20001]]);
                    assert_eq!(result.is_err(), failing);
                    if let Ok(next) = result {
                        assert_eq!(next, state & 1);
                    }
                }
            }
        }
    }
    #[test]
    fn alpha_stops_at_each_failed_effect() {
        let all = [0xc004140c, 0xc0081425, 0xc0041415];
        for failures in 0..8 {
            let fail: Vec<_> = all
                .into_iter()
                .enumerate()
                .filter_map(|(bit, call)| (failures & (1 << bit) != 0).then_some(call))
                .collect();
            let mut fake = Fake {
                calls: vec![],
                fail,
                arguments: vec![],
            };
            assert_eq!(
                configure_alpha(&mut fake, 0x20001, 0x40001).is_ok(),
                failures == 0
            );
            let expected_len = (0..3)
                .find(|bit| failures & (1 << bit) != 0)
                .map_or(3, |bit| bit + 1);
            assert_eq!(fake.calls, all[..expected_len]);
        }
    }
    #[test]
    fn invalid_state_never_touches_device() {
        for state in [4, 0xffff, u32::MAX] {
            let mut fake = Fake {
                calls: vec![],
                fail: vec![],
                arguments: vec![],
            };
            assert!(present(&mut fake, 0x20001, 0x40001, 0x80001, state).is_err());
            assert!(hide(&mut fake, 0x20001, state).is_err());
            assert!(fake.calls.is_empty());
        }
    }
}
