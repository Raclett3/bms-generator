use std::f32::consts::PI;

/*
pub fn sinusoid(frequency: f32) -> impl Fn(f32) -> f32 {
    move |second: f32| f32::sin(second * frequency * 2.0 * PI)
}
*/

pub fn triangle(frequency: f32) -> impl Fn(f32) -> f32 {
    move |second: f32| f32::asin(f32::sin(second * frequency * 2.0 * PI))
}

fn f32_sample_to_i16(sample: f32) -> i16 {
    unsafe {
        ((sample) * 32767.0)
            .clamp(-32767.0, 32767.0)
            .to_int_unchecked()
    }
}

pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Envelope {
            attack,
            decay,
            sustain,
            release,
        }
    }

    fn current_volume(&self, second: f32) -> f32 {
        if second < self.attack {
            second / self.attack
        } else if second - self.attack < self.decay {
            let relative_pos = (second - self.attack) / self.decay;
            (1.0 - relative_pos) + relative_pos * self.sustain
        } else {
            self.sustain
        }
    }

    fn release_volume(&self, second: f32) -> f32 {
        if second < self.release {
            (1.0 - second / self.release) * self.sustain
        } else {
            0.0
        }
    }
}

pub fn sample(
    mut oscillator: impl FnMut(f32) -> f32,
    sustain_duration: f32,
    envelope: &Envelope,
    sample_rate: f32,
    volume: f32,
) -> Vec<[i16; 2]> {
    let sustain_samples = (sustain_duration * sample_rate) as usize;
    let release_samples = (envelope.release * sample_rate) as usize;

    (0..(sustain_samples + release_samples))
        .map(|i| {
            let second = i as f32 / sample_rate;
            let envelope_volume = if i < sustain_samples {
                envelope.current_volume(i as f32 / sample_rate)
            } else {
                envelope.release_volume((i - sustain_samples) as f32 / sample_rate)
            };
            let sample = f32_sample_to_i16(oscillator(second) * envelope_volume * volume);
            [sample, sample]
        })
        .collect()
}
