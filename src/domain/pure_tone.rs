use dasp::{signal, Signal};

pub struct PureTones {
    pub samples: Vec<i16>,
}

impl PureTones {
    pub fn new(sample_rate: u32, track: Vec<ToneAndDuration>) -> Self {
        let total_frames = (sample_rate as f32
            * track
                .iter()
                .map(|tone_and_duration| tone_and_duration.duration)
                .sum::<f32>()) as usize;

        let fade_samples = (sample_rate as f32 * 0.01) as usize;
        let signal = signal::from_iter(track.iter().flat_map(|tone_and_duration| {
            let duration_samples = (sample_rate as f32 * tone_and_duration.duration) as usize;

            signal::rate(sample_rate as f64)
                .const_hz(tone_and_duration.frequency.into())
                .sine()
                .take((sample_rate as f32 * tone_and_duration.duration) as usize)
                .enumerate()
                .map(move |(i, sample)| {
                    let fade_factor = if i < fade_samples {
                        i as f64 / fade_samples as f64
                    } else if i >= duration_samples - fade_samples {
                        (duration_samples - i) as f64 / fade_samples as f64
                    } else {
                        1.0
                    };
                    sample * fade_factor
                })
        }));

        let samples: Vec<i16> = signal
            .take(total_frames)
            .map(|sample| (sample * i16::MAX as f64) as i16)
            .collect();

        Self { samples }
    }
}

#[derive(Debug, PartialEq)]
pub struct ToneAndDuration {
    pub frequency: f32,
    pub duration: f32,
}
