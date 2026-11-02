//! Common volume scale for the supervisor, overlay, and PCM service.
pub const MAX_GAIN: f32 = 0.3;
pub const STEP: f32 = MAX_GAIN / 20.0;
pub const DEFAULT_GAIN: f32 = MAX_GAIN / 4.0;

pub fn percent(gain: f32) -> u32 {
    (gain.clamp(0.0, MAX_GAIN) / MAX_GAIN * 100.0).round() as u32
}
