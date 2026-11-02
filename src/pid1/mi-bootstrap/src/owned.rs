//! Composition of the owned cold-start stages, with no vendor userspace calls.
//! The caller stages and verifies configuration/modules before supplying the
//! cold-boot permission, and retains the returned owner for the entire boot.
use crate::{
    amp_handles, amplifier, cold_start, ColdBootProvenance, Device, DisplayHandoff, Module,
    Observation, Transport,
};
use std::io;

pub struct DisplayHardware {
    device: Device,
    handoff: DisplayHandoff,
    observations: Vec<Observation>,
    audio_attempted: bool,
}

fn require_uninitialized(io: &mut impl Transport, modules: &[Module]) -> io::Result<()> {
    for &module in modules {
        if io.status(module.status_selector())? {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!(
                    "explicit cold profile requires {} to be uninitialized",
                    module.name()
                ),
            ));
        }
    }
    Ok(())
}

impl DisplayHardware {
    /// Initialize only display hardware first. No audio, scanout allocation,
    /// readiness publication, shared-library loading or process spawning.
    pub fn initialize(mut device: Device, cold: ColdBootProvenance) -> io::Result<Self> {
        device.permit_initialization(cold)?;
        require_uninitialized(&mut device, cold_start::DISPLAY_SEQUENCE)?;
        let observations = crate::initialize(&mut device, cold_start::DISPLAY_SEQUENCE)?;
        let handoff = device.finish_display_bootstrap()?;
        Ok(Self {
            device,
            handoff,
            observations,
            audio_attempted: false,
        })
    }

    pub fn controller(&self) -> u32 {
        self.handoff.controller
    }
    pub fn observations(&self) -> &[Observation] {
        &self.observations
    }

    /// Add cold audio/GPIO/IIC initialization and program the amplifier muted.
    /// The plan must be hash-verified from this board's extracted configuration.
    /// No PCM stream is opened and no speaker unmute occurs here.
    /// A failure leaves the display owner available and prohibits another audio
    /// attempt during this boot. A successful audio owner borrows the display
    /// owner so it cannot accidentally close its underlying Device first.
    pub fn initialize_audio(&mut self, plan: &amplifier::Plan) -> io::Result<AudioHardware<'_>> {
        if self.audio_attempted {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "audio bootstrap already attempted",
            ));
        }
        self.audio_attempted = true;
        require_uninitialized(&mut self.device, cold_start::AUDIO_SEQUENCE)?;
        let audio = crate::initialize(&mut self.device, cold_start::AUDIO_SEQUENCE)?;
        let handles = {
            let io = amp_handles::DeviceIo::new(&mut self.device);
            amp_handles::Session::new(io).prepare()?
        };
        // SAFETY: this owner holds the exclusive cold-boot permit. No vendor
        // process or PCM client has been started by any stage above. Handles
        // were verified by the same owning Device immediately before adoption.
        let bus = unsafe { amplifier::MiBus::adopt_initialized_quiescent()? };
        let mut amplifier = amplifier::Ad82111::new(bus);
        amplifier.initialize_muted(plan)?;
        let all_io = crate::speaker::NativeIo::from_cold_device(&mut self.device)?;
        let speaker_io = crate::speaker::NativeIo::from_cold_device(&mut self.device)?;
        let outputs = crate::speaker::Outputs::create(all_io, speaker_io)?;
        Ok(AudioHardware {
            outputs,
            display: self,
            amplifier,
            audio,
            handles,
        })
    }
}

pub struct AudioHardware<'a> {
    // Declared first so output closes while amplifier and Device are still live.
    outputs: crate::speaker::Outputs<crate::speaker::NativeIo>,
    display: &'a DisplayHardware,
    amplifier: amplifier::Ad82111<amplifier::MiBus>,
    audio: Vec<Observation>,
    handles: amp_handles::Handles,
}
impl AudioHardware<'_> {
    pub fn speaker_handle(&self) -> u32 {
        self.outputs.speaker_handle()
    }
    pub fn all_output_handle(&self) -> u32 {
        self.outputs.all_handle()
    }
    pub fn display(&self) -> &DisplayHardware {
        &self.display
    }
    pub fn audio_observations(&self) -> &[Observation] {
        &self.audio
    }
    pub fn amplifier_handles(&self) -> amp_handles::Handles {
        self.handles
    }

    /// Keep the speaker muted while clients and the bounded software gain are
    /// being set up. This does not alter the GPIO reset or kernel MI lifetime.
    pub fn mute(&mut self) -> io::Result<()> {
        self.amplifier.mute()
    }

    /// Call only after the PCM path is silent and its software gain is bounded.
    pub fn unmute(&mut self) -> io::Result<()> {
        self.amplifier.unmute()
    }
}
