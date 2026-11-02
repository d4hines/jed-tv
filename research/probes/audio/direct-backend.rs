//! Single-owner direct PCM backend; warm initialized hardware only.
use crate::{
    aout_route::{
        Aout, AttachError, InputHandle, OwnedRoute, OwnedVolume, RoutePolicy, Snapshot, VolumeDb,
        VolumeTakeoverError,
    },
    mi_pcm::{DeviceTransport, Pcm, Transport},
    pcm_converter::{consumed_input_frames, Converter},
    protocol,
};
use std::{
    io,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
#[derive(Default)]
pub struct DirectBackend {
    owner: Arc<AtomicU8>,
    speaker_route: RoutePolicy,
}
// 0 available, 1 owned, 2 cleanup failed: explicit recovery required.
struct Lease(Arc<AtomicU8>);
impl Lease {
    fn poison(&self) {
        self.0.store(2, Ordering::Release)
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        let _ = self
            .0
            .compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire);
    }
}
pub struct DirectTrack<P: Transport = DeviceTransport, A: Transport = DeviceTransport> {
    pcm: Option<Pcm<P>>,
    route: Option<OwnedRoute<Arc<A>>>,
    volume: Option<OwnedVolume<Arc<A>>>,
    unresolved_route: Option<Snapshot>,
    _lease: Lease,
    converter: Converter,
    rate: u32,
    channels: u32,
    input_written: u64,
    output_written: u64,
}
impl protocol::Backend for DirectBackend {
    type Track = DirectTrack;
    fn supports(&self, rate: u32, channels: u32) -> bool {
        [44100, 48000].contains(&rate) && [1, 2].contains(&channels)
    }
    fn open(&self, rate: u32, channels: u32) -> io::Result<DirectTrack> {
        self.open_using(rate, channels, Pcm::open, Aout::existing_speaker)
    }
    fn name(&self) -> &'static str {
        "direct MI PCM, warm hardware, 48k stereo output"
    }
}
impl<P: Transport, A: Transport> protocol::Stream for DirectTrack<P, A> {
    fn write(&mut self, samples: &[i16]) -> io::Result<()> {
        let output = self.converter.convert(samples)?;
        let mut at = 0;
        let deadline = Instant::now() + Duration::from_secs(5);
        while at < output.len() {
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "direct PCM write deadline",
                ));
            }
            // Do not issue a 1MiB hardware write: bound each nonblocking ioctl to 10ms.
            let end = (at + 1920).min(output.len());
            let n = self.pcm.as_mut().unwrap().write(&output[at..end])?;
            at += n;
            self.output_written += (n / 4) as u64;
            if n == 0 {
                std::thread::sleep(Duration::from_millis(2))
            }
        }
        self.input_written += (samples.len() / self.channels as usize) as u64;
        Ok(())
    }
    fn written(&self) -> u64 {
        self.input_written
    }
    fn played(&self) -> io::Result<u64> {
        let buffer = self.pcm.as_ref().unwrap().buffer_info()?;
        if buffer.queued_bytes % 4 != 0 || buffer.queued_bytes as u64 / 4 > self.output_written {
            return Err(io::Error::other("inconsistent hardware PCM queue"));
        }
        let output_played = self.output_written - buffer.queued_bytes as u64 / 4;
        Ok(consumed_input_frames(
            output_played,
            self.rate,
            self.input_written,
        ))
    }
}
impl<P: Transport, A: Transport> DirectTrack<P, A> {
    /// One nonblocking, frame-aligned ioctl. The Lean executor owns chunking,
    /// conversion, retries, deadlines and counters for this entry point.
    pub fn write_raw(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.pcm.as_mut().ok_or_else(|| io::Error::other("PCM closed"))?.write(bytes)
    }
    /// One raw queue observation; interpreted and validated by Lean.
    pub fn queued_bytes(&self) -> io::Result<u32> {
        Ok(self.pcm.as_ref().ok_or_else(|| io::Error::other("PCM closed"))?.buffer_info()?.queued_bytes)
    }
    /// Explicit shutdown reports errors; Drop also performs this sequence.
    pub fn finish(mut self) -> io::Result<()> {
        self.cleanup()
    }
    fn cleanup(&mut self) -> io::Result<()> {
        let mut failures = vec![];
        if let Some(volume) = self.volume.take() {
            let original = volume.original();
            if let Err(error) = volume.restore() {
                failures.push((
                    "speaker gain restore",
                    io::Error::other(format!("restore {original:?}: {error}")),
                ));
            }
        }
        // Mandatory lifetime rule: PCM must remain alive while any AOUT reference
        // may remain. Failed restoration retains the PCM until explicit recovery.
        if let Some(route) = self.route.take() {
            let original = route.before().clone();
            if let Err(error) = route.restore() {
                self.unresolved_route = Some(original);
                failures.push(("route restore", error));
            }
        }
        if let Some(original) = self.unresolved_route.take() {
            self._lease.poison();
            let retained_pcm_handle = self.pcm.take().map(|pcm| {
                let handle = pcm.handle();
                // A known intentional leak: dropping/closing could free memory
                // still referenced by AOUT. Process-level recovery is explicit.
                std::mem::forget(pcm);
                handle
            });
            return Err(io::Error::other(DirectCleanupError {
                failures,
                retained_pcm_handle,
                original_route: Some(original),
            }));
        }
        if let Some(pcm) = self.pcm.take() {
            if let Err(error) = pcm.finish() {
                failures.push(("PCM close", error));
            }
        }
        if failures.is_empty() {
            Ok(())
        } else {
            self._lease.poison();
            Err(io::Error::other(DirectCleanupError {
                failures,
                retained_pcm_handle: None,
                original_route: None,
            }))
        }
    }
}
#[derive(Debug)]
pub struct DirectCleanupError {
    pub retained_pcm_handle: Option<u32>,
    pub original_route: Option<Snapshot>,
    pub failures: Vec<(&'static str, io::Error)>,
}
impl std::fmt::Display for DirectCleanupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "direct audio cleanup failed")?;
        if let Some(handle) = self.retained_pcm_handle {
            write!(
                f,
                "; retained PCM {handle:#x}, original route {:?}; explicit recovery required",
                self.original_route
            )?;
        }
        for (stage, error) in &self.failures {
            write!(f, "; {stage}: {error}")?;
        }
        Ok(())
    }
}
impl std::error::Error for DirectCleanupError {}
impl<P: Transport, A: Transport> Drop for DirectTrack<P, A> {
    fn drop(&mut self) {
        if let Err(error) = self.cleanup() {
            eprintln!("{error}")
        }
    }
}
impl DirectBackend {
    /// Acquire fixed 48k stereo PCM and its existing exclusive route/volume lease.
    pub fn open_raw(&self) -> io::Result<DirectTrack> {
        self.open_using(48000, 2, Pcm::open, Aout::existing_speaker)
    }
    pub fn with_speaker_route(policy: RoutePolicy) -> Self {
        Self {
            speaker_route: policy,
            ..Default::default()
        }
    }
    fn open_using<P: Transport, A: Transport>(
        &self,
        rate: u32,
        channels: u32,
        open_pcm: impl FnOnce() -> io::Result<Pcm<P>>,
        open_aout: impl FnOnce() -> io::Result<Aout<A>>,
    ) -> io::Result<DirectTrack<P, A>> {
        let converter = Converter::new(rate, channels)?;
        let lease = self.acquire()?;
        let pcm = open_pcm()?;
        let mut track = DirectTrack {
            pcm: Some(pcm),
            route: None,
            volume: None,
            unresolved_route: None,
            _lease: lease,
            converter,
            rate,
            channels,
            input_written: 0,
            output_written: 0,
        };
        {
            let policy = self.speaker_route;
            let aout = open_aout()?.shared();
            let gain_aout = aout.duplicate();
            let input = InputHandle::from_raw(track.pcm.as_ref().unwrap().handle())?;
            match aout.route_owned(input, policy) {
                Ok(route) => track.route = Some(route),
                Err(error) => {
                    if error
                        .get_ref()
                        .and_then(|e| e.downcast_ref::<AttachError>())
                        .is_some_and(|e| e.rollback.is_some())
                    {
                        track._lease.poison();
                        track.unresolved_route = error
                            .get_ref()
                            .and_then(|e| e.downcast_ref::<AttachError>())
                            .map(|e| e.original.clone());
                    }
                    return Err(error);
                }
            }
            match gain_aout.volume_owned(VolumeDb {
                integer: 0,
                thousandths: 0,
            }) {
                Ok(volume) => track.volume = Some(volume),
                Err(error) => {
                    if error
                        .get_ref()
                        .and_then(|e| e.downcast_ref::<VolumeTakeoverError>())
                        .is_some_and(|e| e.rollback.is_some())
                    {
                        track._lease.poison();
                    }
                    return Err(error);
                }
            }
        }
        track.pcm.as_mut().unwrap().start()?;
        Ok(track)
    }
    fn acquire(&self) -> io::Result<Lease> {
        match self
            .owner
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => Ok(Lease(self.owner.clone())),
            Err(2) => Err(io::Error::other(
                "direct PCM cleanup failed; explicit recovery required before reuse",
            )),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "direct PCM is already owned",
            )),
        }
    }
}

#[cfg(test)]
#[path = "../../../test/unit/audio/direct-backend-tests.rs"]
mod tests;
