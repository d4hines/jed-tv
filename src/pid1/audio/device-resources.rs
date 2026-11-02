//! Individual MI handle operations for TV.Audio.Device. Lean chooses their
//! order and owns exclusivity; this holder neither retries nor calls Lean.
use crate::{
    aout_route::{
        Aout, AttachError, InputHandle, OwnedRoute, OwnedVolume, RoutePolicy, VolumeDb,
        VolumeTakeoverError,
    },
    mi_pcm::{DeviceTransport, Pcm, Transport},
};
use std::{io, sync::Arc};

pub struct Failure {
    pub error: io::Error,
    pub uncertain: bool,
}
impl From<io::Error> for Failure {
    fn from(error: io::Error) -> Self {
        Self {
            error,
            uncertain: false,
        }
    }
}
fn invalid(message: &'static str) -> Failure {
    io::Error::other(message).into()
}

pub struct Resources<P: Transport = DeviceTransport, A: Transport = DeviceTransport> {
    pcm: Option<Pcm<P>>,
    output: Option<Aout<Arc<A>>>,
    route: Option<OwnedRoute<Arc<A>>>,
    volume: Option<OwnedVolume<Arc<A>>>,
    open_pcm: Box<dyn Fn() -> io::Result<Pcm<P>>>,
    open_output: Box<dyn Fn() -> io::Result<Aout<A>>>,
}
impl Default for Resources {
    fn default() -> Self {
        Self::with_factories(Pcm::open, Aout::existing_speaker)
    }
}
impl<P: Transport, A: Transport> Resources<P, A> {
    pub fn with_factories(
        open_pcm: impl Fn() -> io::Result<Pcm<P>> + 'static,
        open_output: impl Fn() -> io::Result<Aout<A>> + 'static,
    ) -> Self {
        Self {
            pcm: None,
            output: None,
            route: None,
            volume: None,
            open_pcm: Box::new(open_pcm),
            open_output: Box::new(open_output),
        }
    }
    /// Scalar ABI matches the ten typed effects in TV.Audio.Device.
    pub fn step(&mut self, operation: u32) -> Result<(), Failure> {
        match operation {
            0 => {
                if self.pcm.is_some() {
                    return Err(invalid("PCM already open"));
                }
                self.pcm = Some((self.open_pcm)()?);
            }
            1 => {
                if self.output.is_some() {
                    return Err(invalid("output already open"));
                }
                self.output = Some((self.open_output)()?.shared());
            }
            2 => {
                if self.route.is_some() {
                    return Err(invalid("route already owned"));
                }
                let input = InputHandle::from_raw(
                    self.pcm
                        .as_ref()
                        .ok_or_else(|| invalid("PCM closed"))?
                        .handle(),
                )?;
                let output = self
                    .output
                    .as_ref()
                    .ok_or_else(|| invalid("output closed"))?
                    .duplicate();
                self.route = Some(
                    output
                        .route_owned(input, RoutePolicy::RequireEmpty)
                        .map_err(|error| {
                            let uncertain = error
                                .get_ref()
                                .and_then(|e| e.downcast_ref::<AttachError>())
                                .is_some_and(|e| e.rollback.is_some());
                            Failure { error, uncertain }
                        })?,
                );
            }
            3 => {
                if self.volume.is_some() {
                    return Err(invalid("volume already owned"));
                }
                let output = self
                    .output
                    .as_ref()
                    .ok_or_else(|| invalid("output closed"))?
                    .duplicate();
                self.volume = Some(
                    output
                        .volume_owned(VolumeDb {
                            integer: 0,
                            thousandths: 0,
                        })
                        .map_err(|error| {
                            let uncertain = error
                                .get_ref()
                                .and_then(|e| e.downcast_ref::<VolumeTakeoverError>())
                                .is_some_and(|e| e.rollback.is_some());
                            Failure { error, uncertain }
                        })?,
                );
            }
            4 => self
                .pcm
                .as_mut()
                .ok_or_else(|| invalid("PCM closed"))?
                .start()?,
            5 => {
                if let Some(volume) = self.volume.take() {
                    volume.restore()?;
                }
            }
            6 => {
                if let Some(route) = self.route.take() {
                    route.restore()?;
                }
            }
            7 => {
                if self.route.is_some() || self.volume.is_some() {
                    return Err(Failure {
                        error: io::Error::other("PCM still referenced by output ownership"),
                        uncertain: true,
                    });
                }
                if let Some(pcm) = self.pcm.take() {
                    pcm.finish()?;
                }
            }
            8 => {
                if let Some(pcm) = self.pcm.take() {
                    std::mem::forget(pcm);
                }
            }
            9 => {
                if self.pcm.is_some() || self.route.is_some() || self.volume.is_some() {
                    return Err(Failure {
                        error: io::Error::other("device resources remain owned"),
                        uncertain: true,
                    });
                }
                self.output = None;
            }
            _ => return Err(invalid("unknown audio device operation")),
        }
        Ok(())
    }
    pub fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.pcm
            .as_mut()
            .ok_or_else(|| io::Error::other("PCM closed"))?
            .write(bytes)
    }
    pub fn queued_bytes(&self) -> io::Result<u32> {
        Ok(self
            .pcm
            .as_ref()
            .ok_or_else(|| io::Error::other("PCM closed"))?
            .buffer_info()?
            .queued_bytes)
    }
    /// Unexpected native/Lean failure cannot run a guessed cleanup sequence.
    /// Retain live handles until process recovery instead of freeing referenced DMA.
    pub fn abandon(&mut self) {
        if let Some(volume) = self.volume.take() {
            std::mem::forget(volume);
        }
        if let Some(route) = self.route.take() {
            std::mem::forget(route);
        }
        if let Some(pcm) = self.pcm.take() {
            std::mem::forget(pcm);
        }
    }
}
impl<P: Transport, A: Transport> Drop for Resources<P, A> {
    fn drop(&mut self) {
        self.abandon();
    }
}

#[cfg(test)]
#[path = "../../../test/unit/audio/device-resources-tests.rs"]
mod tests;
