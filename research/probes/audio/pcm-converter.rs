//! Causal linear PCM conversion to 48 kHz stereo, with exact rational phase.
//! One previous input frame is retained; chunk boundaries do not reset phase.
use std::io;
pub struct Converter {
    rate: u32,
    channels: usize,
    phase: u32,
    previous: Option<[i16; 2]>,
}
impl Converter {
    pub fn new(rate: u32, channels: u32) -> io::Result<Self> {
        if ![44100, 48000].contains(&rate) || ![1, 2].contains(&channels) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "direct audio supports 44.1/48k mono/stereo",
            ));
        }
        Ok(Self {
            rate,
            channels: channels as usize,
            phase: 0,
            previous: None,
        })
    }
    pub fn convert(&mut self, input: &[i16]) -> io::Result<Vec<u8>> {
        if input.len() % self.channels != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "incomplete PCM frame",
            ));
        }
        let mut output = Vec::with_capacity((input.len() / self.channels + 1) * 8);
        for frame in input.chunks_exact(self.channels) {
            let current = [frame[0], frame[self.channels - 1]];
            let previous = self.previous.unwrap_or(current);
            self.phase += 48000;
            while self.phase >= self.rate {
                self.phase -= self.rate;
                let fraction = (48000 - self.phase) as i64;
                for channel in 0..2 {
                    let value = (previous[channel] as i64 * (48000 - fraction)
                        + current[channel] as i64 * fraction)
                        / 48000;
                    output.extend_from_slice(&(value as i16).to_le_bytes());
                }
            }
            self.previous = Some(current);
        }
        Ok(output)
    }
}
/// Number of whole input frames whose generated output has all been consumed.
/// Inverse of floor(input_frames * 48000 / rate); saturate to accepted input.
pub fn consumed_input_frames(output: u64, rate: u32, written: u64) -> u64 {
    ((((output as u128 + 1) * rate as u128 - 1) / 48000) as u64).min(written)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stereo_48k_is_bit_exact() {
        let data = [i16::MIN, i16::MAX, -1, 1, 100, -100, 0, 0];
        let out = Converter::new(48000, 2).unwrap().convert(&data).unwrap();
        assert_eq!(
            out,
            data.iter()
                .flat_map(|v| v.to_le_bytes())
                .collect::<Vec<_>>()
        );
    }
    #[test]
    fn mono_is_duplicated() {
        let out = Converter::new(48000, 1)
            .unwrap()
            .convert(&[-9, 0, 11])
            .unwrap();
        assert_eq!(
            out,
            [-9i16, -9, 0, 0, 11, 11]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect::<Vec<_>>()
        );
    }
    #[test]
    fn long_run_count_and_chunk_independence() {
        for channels in [1, 2] {
            let data: Vec<i16> = (0..44100 * channels * 7)
                .map(|i| (((i * 113) % 65536) as i32 - 32768) as i16)
                .collect();
            let whole = Converter::new(44100, channels as u32)
                .unwrap()
                .convert(&data)
                .unwrap();
            assert_eq!(whole.len() / 4, 48000 * 7);
            let mut split = Converter::new(44100, channels as u32).unwrap();
            let mut output = vec![];
            let mut at = 0;
            let mut turn = 0;
            while at < data.len() {
                let n = ((turn * 17) % 373 + 1) * channels;
                let end = (at + n).min(data.len());
                output.extend(split.convert(&data[at..end]).unwrap());
                at = end;
                turn += 1;
            }
            assert_eq!(output, whole);
        }
    }
    #[test]
    fn cumulative_floor_count_and_inverse() {
        let mut converter = Converter::new(44100, 1).unwrap();
        let mut count = 0u64;
        for n in 1..=10000u64 {
            count += (converter.convert(&[1]).unwrap().len() / 4) as u64;
            assert_eq!(count, n * 48000 / 44100);
            assert_eq!(consumed_input_frames(count, 44100, n), n);
            if count > 0 {
                assert!(consumed_input_frames(count - 1, 44100, n) < n)
            }
        }
        assert_eq!(consumed_input_frames(0, 44100, 100), 0);
    }
    #[test]
    fn interpolation_stays_bounded_and_silent_input_stays_zero() {
        let input = [i16::MIN, i16::MAX, i16::MIN, i16::MAX, 0, i16::MAX];
        let out = Converter::new(44100, 1).unwrap().convert(&input).unwrap();
        for sample in out.chunks_exact(4) {
            assert_eq!(&sample[..2], &sample[2..]);
        }
        assert!(Converter::new(44100, 2)
            .unwrap()
            .convert(&[0; 882])
            .unwrap()
            .iter()
            .all(|b| *b == 0));
    }
    #[test]
    fn rejects_unsupported_or_incomplete_frames_without_phase_change() {
        assert!(Converter::new(32000, 2).is_err());
        assert!(Converter::new(48000, 3).is_err());
        let mut converter = Converter::new(44100, 2).unwrap();
        assert!(converter.convert(&[7]).is_err());
        assert_eq!(
            converter.convert(&[7, 8]).unwrap(),
            Converter::new(44100, 2).unwrap().convert(&[7, 8]).unwrap()
        );
    }
}
