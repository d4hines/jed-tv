//! City flight demo (github.com/uyhcire/vibecheck-tsa) on the TV: a Cessna 172
//! over a procedural city, with the project's safety filter that stops the
//! pilot from crashing. On the TV the aircraft is JsbExact, their Lean port of
//! JSBSim, and the filter checks exact rollouts on it (both cross-built for
//! 32-bit ARM). Everything runs on the TV; nothing is streamed.
//!
//! The simulation runs in slow motion when the TV cannot keep up: each frame
//! advances the game by whole 1/50 s ticks, as many as fit in a frame budget.

pub mod render;
pub mod state;

use cityflight::model::{State, GAM, PHI, PSI, THR, V, X, Y, Z};
use cityflight::pilot::{Fbw, Kamikaze};
use cityflight::sim::{scenario_start, Sim, SimCfg, SCENARIOS};
use cityflight::world::World;
use render::{panel, text, OUT_W};
use std::time::Duration;

/// One 1/50 s control tick of the demo.
pub const DT_F: f64 = cityflight::model::DT_F;

pub struct Flight {
    pub world: &'static World,
    pub sim: Sim<'static>,
    pub fbw: Fbw,
    pub scenario: usize,
    pub seed: u64,
    /// Smoothed stick (pitch, roll) in -1..1.
    pub stick: (f64, f64),
    pub kamikaze: Option<Kamikaze>,
    pub crashes: u32,
}

impl Flight {
    pub fn new(world: &'static World, scenario: usize) -> Flight {
        let s0 = scenario_start(world, scenario);
        Flight {
            world, sim: Sim::new(world, s0, SimCfg::default()), fbw: Fbw::from_state(&s0), scenario, seed: 1,
            stick: (0.0, 0.0), kamikaze: None, crashes: 0,
        }
    }

    pub fn reset(&mut self, scenario: usize) {
        self.scenario = scenario;
        self.seed += 1;
        let s0 = scenario_start(self.world, scenario);
        let on = self.sim.cfg.filter_on;
        self.sim = Sim::new(self.world, s0, SimCfg { filter_on: on, seed: self.seed, ..SimCfg::default() });
        self.fbw = Fbw::from_state(&s0);
        self.stick = (0.0, 0.0);
        if self.kamikaze.is_some() {
            self.kamikaze = Some(Kamikaze::new(self.seed));
        }
    }

    pub fn toggle_filter(&mut self) {
        let on = !self.sim.cfg.filter_on;
        self.sim.set_filter(on);
    }

    pub fn toggle_kamikaze(&mut self) {
        self.kamikaze = if self.kamikaze.is_some() { None } else { Some(Kamikaze::new(self.seed)) };
    }

    pub fn throttle(&mut self, delta: f64) {
        self.fbw.thr = (self.fbw.thr + delta).clamp(0.0, 1.0);
    }

    /// Move the stick toward the held direction (pitch > 0 = nose up, roll > 0 = right).
    pub fn steer(&mut self, pitch: f64, roll: f64, dt: f64) {
        let k = (dt * 8.0).min(1.0);
        self.stick.0 += (pitch - self.stick.0) * k;
        self.stick.1 += (roll - self.stick.1) * k;
    }

    /// One control tick: pilot (or the kamikaze autopilot) commands in, physics and filter out.
    pub fn tick(&mut self) {
        if self.sim.crashed {
            return;
        }
        if let Some(k) = &mut self.kamikaze {
            k.update(self.world, &self.sim.s, &mut self.fbw, DT_F);
        } else {
            self.fbw.sticks(self.stick.0, self.stick.1, DT_F);
        }
        let u = self.fbw.cmd(&self.sim.prm_ctrl, &self.sim.s);
        self.sim.tick(u);
        self.fbw.resync(&self.sim.s, self.sim.fs.lam, DT_F);
        if self.sim.crashed {
            self.crashes += 1;
        }
    }

    /// Everything the screen needs, copied out so drawing can run on another thread.
    pub fn snap(&self, tick_ms: f64) -> Snap {
        let (path, accepted, escape) = self.escape();
        let s = self.sim.s;
        Snap {
            s, time: self.sim.time(), scenario: self.scenario, filter_on: self.sim.cfg.filter_on, crashed: self.sim.crashed,
            kamikaze: self.kamikaze.is_some(), agl: s[Z] - self.world.surface(s[X], s[Y]), path, accepted, escape, tick_ms,
        }
    }

    /// The filter's verified escape path and whether the pilot's input was accepted on the last tick.
    pub fn escape(&self) -> (Vec<[f64; 3]>, bool, String) {
        #[cfg(target_os = "linux")]
        if let Some(g) = &self.sim.env_gate {
            let name = g.witness.map(|i| cityflight::envfilter::ESCAPES[i].name).unwrap_or("none verified");
            let flying = g.law.is_some();
            let ms = *g.cpu_ms.last().unwrap_or(&0.0);
            let status = if flying { format!("filter flying escape: {name} ({ms:.0} ms)") } else { format!("verified escape: {name} ({ms:.0} ms)") };
            return (g.witness_path.clone(), !flying, status);
        }
        let mode = format!("{:?}", self.sim.fs.mode).to_lowercase();
        (Vec::new(), true, format!("point-mass filter: {mode}"))
    }

    pub fn state(&self) -> &State {
        &self.sim.s
    }
}

/// A copy of the simulation for the screen.
#[derive(Clone, Debug)]
pub struct Snap {
    pub s: State,
    pub time: f64,
    pub scenario: usize,
    pub filter_on: bool,
    pub crashed: bool,
    pub kamikaze: bool,
    /// Height above the ground or building below.
    pub agl: f64,
    /// The filter's verified escape path, whether the pilot is in control, and the filter's status line.
    pub path: Vec<[f64; 3]>,
    pub accepted: bool,
    pub escape: String,
    /// Cost of the last control tick (ms), mostly the filter's exact rollouts.
    pub tick_ms: f64,
}

/// What the frame loop knows that the simulation does not.
pub struct Overlay<'a> {
    pub fps: f64,
    /// Simulated seconds per real second.
    pub speed: f64,
    pub show_help: bool,
    pub status: &'a str,
}

/// Draw the HUD over a 1280x720 frame.
pub fn hud(out: &mut [u32], snap: &Snap, o: &Overlay) {
    let s = &snap.s;
    let white = 0xf4f4f8;
    let grey = 0xb0b4c0;
    let green = 0x40e070;
    let red = 0xf04030;
    let amber = 0xf0c040;
    let heading = (s[PSI].to_degrees().rem_euclid(360.0)) as i32;
    panel(out, 16, 16, 470, 118);
    text(out, 28, 26, 2, white, &format!("SPD {:>3.0} m/s {:>3.0} kt   ALT {:>4.0} m AGL", s[V], s[V] * 1.944, snap.agl));
    text(out, 28, 50, 2, white, &format!("HDG {heading:>3}   BANK {:>4.0}   CLIMB {:>+3.0}", s[PHI].to_degrees(), s[GAM].to_degrees()));
    text(out, 28, 74, 2, grey, &format!("THROTTLE {:>3.0}%   T {:>5.1} s   {}", s[THR] * 100.0, snap.time, SCENARIOS[snap.scenario]));
    text(out, 28, 98, 2, grey, &format!("{:.1} fps   sim {:.0}% of real time   tick {:.0} ms", o.fps, o.speed * 100.0, snap.tick_ms));
    let right = OUT_W as i32;
    let filter = if snap.filter_on { "SAFETY FILTER ON" } else { "SAFETY FILTER OFF" };
    let fw = 8 * 2 * filter.len() as i32;
    panel(out, right - 44 - fw, 16, fw + 28, 30);
    text(out, right - 30 - fw, 26, 2, if snap.filter_on { green } else { red }, filter);
    let ew = 8 * 2 * snap.escape.len() as i32;
    panel(out, right - 44 - ew, 48, ew + 28, 30);
    text(out, right - 30 - ew, 56, 2, if snap.accepted { grey } else { amber }, &snap.escape);
    if snap.kamikaze {
        let w = 8 * 2 * 32;
        text(out, right - 30 - w, 86, 2, amber, "KAMIKAZE AUTOPILOT (filter test)");
    }
    if snap.crashed {
        panel(out, 340, 300, 600, 110);
        text(out, 420, 318, 4, red, "CRASHED");
        text(out, 372, 366, 2, white, "OK: fly this scenario again    YouTube: the next one");
    }
    if o.show_help {
        panel(out, 16, 560, 860, 140);
        let lines = ["Arrows: pitch and roll     OK: safety filter on/off",
                     "Ch+/Ch-: throttle    YouTube: next scenario",
                     "NETFLIX: kamikaze autopilot    prime video: camera",
                     "Back: exit    (keyboard: 1-5 scenarios, Info: help)"];
        for (i, l) in lines.iter().enumerate() {
            text(out, 28, 570 + i as i32 * 30, 2, grey, l);
        }
    }
    if !o.status.is_empty() {
        text(out, 16, 700, 2, grey, o.status);
    }
}

/// A request from the remote to the simulation thread.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    ToggleFilter,
    Throttle(f64),
    Reset(usize),
    ToggleKamikaze,
}

impl Flight {
    pub fn apply(&mut self, c: Command) {
        match c {
            Command::ToggleFilter => self.toggle_filter(),
            Command::Throttle(d) => self.throttle(d),
            Command::Reset(n) => self.reset(n),
            Command::ToggleKamikaze => self.toggle_kamikaze(),
        }
    }
}

/// How long to wait before the next tick so the game does not run faster than real time.
pub fn pace(tick_started: Duration, now: Duration) -> Duration {
    let dt = Duration::from_secs_f64(DT_F);
    dt.saturating_sub(now.saturating_sub(tick_started))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pacing_never_exceeds_real_time() {
        let ms = Duration::from_millis;
        assert_eq!(pace(ms(100), ms(105)), ms(15));
        assert_eq!(pace(ms(100), ms(900)), Duration::ZERO);
    }

    #[test]
    fn point_mass_flight_runs_and_commands_apply() {
        let world: &'static World = Box::leak(Box::new(World::new(1337)));
        let mut f = Flight::new(world, 0);
        for _ in 0..50 {
            f.tick();
        }
        assert!(f.sim.time() > 0.9);
        f.apply(Command::ToggleFilter);
        assert!(!f.sim.cfg.filter_on);
        f.apply(Command::Throttle(0.5));
        f.apply(Command::Reset(2));
        assert_eq!(f.scenario, 2);
        let snap = f.snap(12.0);
        let mut out = vec![0u32; render::OUT_W * render::OUT_H];
        hud(&mut out, &snap, &Overlay { fps: 1.0, speed: 0.03, show_help: true, status: "test" });
        assert!(out.iter().any(|&p| p == 0xf4f4f8));
    }
}
