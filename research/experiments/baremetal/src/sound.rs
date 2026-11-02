//! virtio-sound: a 440 Hz sine (S16_LE stereo, 48 kHz, amplitude 0.3 as in
//! ytplay's audio self-test), streamed from the main loop. Up to IN_FLIGHT
//! periods are queued; QEMU completes each one when it has been played, so
//! completions pace the producer the way ALSA's WRITEI_FRAMES does.

use crate::hal::HalImpl;
use crate::{println, timer};
use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use virtio_drivers::device::sound::{PcmFeatures, PcmFormat, PcmRate, VirtIOSound};
use virtio_drivers::transport::mmio::MmioTransport;

const RATE: u32 = 48_000;
const PERIOD_FRAMES: usize = 960; // 20 ms
const PERIOD_BYTES: usize = PERIOD_FRAMES * 4;
const IN_FLIGHT: usize = 12; // 240 ms queued (16 fit: 32 descriptors, 2 per period)

pub struct Tone {
    snd: VirtIOSound<HalImpl, MmioTransport<'static>>,
    stream: u32,
    phase: f32,
    period: Vec<u8>,
    queued: VecDeque<u16>,
    submitted: usize,
    total: usize,
    pub played: usize,
    started: bool,
    pub starved: u32,
    t_start: u64,
    pub done: bool,
}

impl Tone {
    pub fn new(transport: MmioTransport<'static>, seconds: f32) -> Option<Tone> {
        let mut snd = VirtIOSound::new(transport).ok()?;
        let streams = snd.output_streams().ok()?;
        let stream = *streams.first()?;
        println!("sound: output streams {:?}, rates {:?}", streams, snd.rates_supported(stream).ok());
        snd.pcm_set_params(stream, (PERIOD_BYTES * IN_FLIGHT) as u32, PERIOD_BYTES as u32,
                           PcmFeatures::empty(), 2, PcmFormat::S16, PcmRate::Rate48000).ok()?;
        snd.pcm_prepare(stream).ok()?;
        let total = (seconds * RATE as f32) as usize / PERIOD_FRAMES;
        println!("sound: S16_LE {} Hz x2, period {} frames, {} periods queued, {} periods total",
                 RATE, PERIOD_FRAMES, IN_FLIGHT, total);
        Some(Tone {
            snd, stream, phase: 0.0, period: vec![0; PERIOD_BYTES], queued: VecDeque::new(),
            submitted: 0, total, played: 0, started: false, starved: 0, t_start: 0, done: false,
        })
    }

    pub fn ack_interrupt(&mut self) {
        self.snd.ack_interrupt();
    }

    fn fill(&mut self) {
        let step = 2.0 * core::f32::consts::PI * 440.0 / RATE as f32;
        for f in self.period.chunks_exact_mut(4) {
            let s = (libm::sinf(self.phase) * 0.3 * i16::MAX as f32) as i16;
            f[..2].copy_from_slice(&s.to_le_bytes());
            f[2..].copy_from_slice(&s.to_le_bytes());
            self.phase += step;
            if self.phase > 2.0 * core::f32::consts::PI {
                self.phase -= 2.0 * core::f32::consts::PI;
            }
        }
    }

    /// Reap played periods and top the queue up. Never blocks.
    pub fn pump(&mut self) {
        if self.done {
            return;
        }
        while let Some(&t) = self.queued.front() {
            if self.snd.pcm_xfer_ok(t).is_err() {
                break;
            }
            self.queued.pop_front();
            self.played += 1;
        }
        if self.started && self.queued.is_empty() && self.submitted < self.total {
            self.starved += 1;
        }
        while self.queued.len() < IN_FLIGHT && self.submitted < self.total {
            self.fill();
            match self.snd.pcm_xfer_nb(self.stream, &self.period) {
                Ok(t) => self.queued.push_back(t),
                Err(e) => {
                    println!("sound: xfer failed: {:?}", e);
                    self.done = true;
                    return;
                }
            }
            self.submitted += 1;
        }
        if !self.started {
            self.snd.pcm_start(self.stream).expect("pcm start");
            self.started = true;
            self.t_start = timer::ticks();
        }
        if self.played == self.total {
            let _ = self.snd.pcm_stop(self.stream);
            let _ = self.snd.pcm_release(self.stream);
            let ms = timer::ticks_to_us(timer::ticks() - self.t_start) / 1000;
            println!("sound: played {} frames of 440 Hz in {} ms; queue ran dry {} times",
                     self.played * PERIOD_FRAMES, ms, self.starved);
            self.done = true;
        }
    }
}
