#![no_std]

pub struct Synth {
    sample_rate: u32,
    sample_count: usize,
    playing: Option<Playing>,
}

pub struct Playing {
    samples_high: u32,
    samples_played: u32,
    samples_total: u32,
    duration: Duration,
}

pub struct Sample(i8);

pub struct Duration(u32);

#[derive(Debug, Copy, Clone)]
pub enum Note {
    A4 = 440,
    ASharp4 = 466,
    B4 = 494,
    C4 = 523,
    CSharp4 = 554,
    D4 = 587,
    DSharp4 = 622,
    E4 = 659,
    F4 = 698,
    FSharp4 = 740,
    G4 = 784,
    GSharp4 = 831,
    A5 = 880,
}

impl Synth {
    pub fn new(sample_rate: u32) -> Synth {
        Synth {
            sample_rate,
            sample_count: 0,
            playing: None,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn sample_count(&self) -> usize {
        self.sample_count
    }

    pub fn play(&mut self, note: Note, duration: Duration) {
        let samples_total = self.sample_rate / (note as u32);
        let samples_high = samples_total / 2;
        let samples_played = 0;
        let p = Playing {
            samples_high,
            samples_played,
            samples_total,
            duration,
        };

        self.playing = Some(p);
    }

    pub fn duration(&self, millis: u32) -> Duration {
        Duration::from_millis(self.sample_rate, millis)
    }

    pub fn next(&mut self) -> Sample {
        self.sample_count += 1;
        let (done, sample) = if let Some(ref mut playing) = &mut self.playing {
            let offset = playing.samples_played % playing.samples_total;
            let sample = if offset < playing.samples_high {
                Sample(100)
            } else {
                Sample(-100)
            };
            playing.samples_played += 1;
            if playing.samples_played == playing.duration.0 {
                (true, sample)
            } else {
                (false, sample)
            }
        } else {
            (false, Sample(0))
        };
        if done {
            self.playing = None;
        }
        sample
    }
}

impl Duration {
    pub fn from_millis(sample_rate: u32, millis: u32) -> Duration {
        Duration(sample_rate * millis / 1000)
    }
}

impl core::convert::Into<u8> for Sample {
    fn into(self) -> u8 {
        self.0 as u8
    }
}
