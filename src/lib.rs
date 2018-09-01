#![no_std]

pub struct Synth {
    sample_rate: u32,
    sample_count: usize,
    playing: Option<Playing>,
}

#[derive(Debug)]
struct Playing {
    samples_high: u32,
    samples_played: u32,
    cycle_length: u32,
    duration: Duration,
}

#[derive(Debug)]
pub struct Sample(i8);

#[derive(Debug)]
pub struct Duration(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Note {
    Rest = 0,
    C0 = 1635,
    CsDb0 = 1732,
    D0 = 1835,
    DsEb0 = 1945,
    E0 = 2060,
    F0 = 2183,
    FsGb0 = 2312,
    G0 = 2450,
    GsAb0 = 2596,
    A0 = 2750,
    AsBb0 = 2914,
    B0 = 3087,
    C1 = 3270,
    CsDb1 = 3465,
    D1 = 3671,
    DsEb1 = 3889,
    E1 = 4120,
    F1 = 4365,
    FsGb1 = 4625,
    G1 = 4900,
    GsAb1 = 5191,
    A1 = 5500,
    AsBb1 = 5827,
    B1 = 6174,
    C2 = 6541,
    CsDb2 = 6930,
    D2 = 7342,
    DsEb2 = 7778,
    E2 = 8241,
    F2 = 8731,
    FsGb2 = 9250,
    G2 = 9800,
    GsAb2 = 10383,
    A2 = 11000,
    AsBb2 = 11654,
    B2 = 12347,
    C3 = 13081,
    CsDb3 = 13859,
    D3 = 14683,
    DsEb3 = 15556,
    E3 = 16481,
    F3 = 17461,
    FsGb3 = 18500,
    G3 = 19600,
    GsAb3 = 20765,
    A3 = 22000,
    AsBb3 = 23308,
    B3 = 24694,
    C4 = 26163,
    CsDb4 = 27718,
    D4 = 29366,
    DsEb4 = 31113,
    E4 = 32963,
    F4 = 34923,
    FsGb4 = 36999,
    G4 = 39200,
    GsAb4 = 41530,
    A4 = 44000,
    AsBb4 = 46616,
    B4 = 49388,
    C5 = 52325,
    CsDb5 = 55437,
    D5 = 58733,
    DsEb5 = 62225,
    E5 = 65925,
    F5 = 69846,
    FsGb5 = 73999,
    G5 = 78399,
    GsAb5 = 83061,
    A5 = 88000,
    AsBb5 = 93233,
    B5 = 98777,
    C6 = 104650,
    CsDb6 = 110873,
    D6 = 117466,
    DsEb6 = 124451,
    E6 = 131851,
    F6 = 139691,
    FsGb6 = 147998,
    G6 = 156798,
    GsAb6 = 166122,
    A6 = 176000,
    AsBb6 = 186466,
    B6 = 197553,
    C7 = 209300,
    CsDb7 = 221746,
    D7 = 234932,
    DsEb7 = 248902,
    E7 = 263702,
    F7 = 279383,
    FsGb7 = 295996,
    G7 = 313596,
    GsAb7 = 332244,
    A7 = 352000,
    AsBb7 = 372931,
    B7 = 395107,
    C8 = 418601,
    CsDb8 = 443492,
    D8 = 469863,
    DsEb8 = 497803,
    E8 = 527404,
    F8 = 558765,
    FsGb8 = 591991,
    G8 = 627193,
    GsAb8 = 664488,
    A8 = 704000,
    AsBb8 = 745862,
    B8 = 790213,
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

    pub fn is_playing(&self) -> bool {
        self.playing.is_some()
    }

    pub fn play(&mut self, note: Note, duration: Duration) {
        self.play_phase(note, duration, 128);
    }

    pub fn play_phase(&mut self, note: Note, duration: Duration, phase: u8) {
        let p = if note == Note::Rest {
            Playing {
                samples_high: 0,
                samples_played: 0,
                cycle_length: 1,
                duration,
            }
        } else {
            // notes are in centi-hertz
            let cycle_length = (self.sample_rate * 100) / (note as u32);
            let samples_high = (cycle_length * phase as u32) / 256;
            let samples_played = 0;
            Playing {
                samples_high,
                samples_played,
                cycle_length,
                duration,
            }
        };
        self.playing = Some(p);
    }

    pub fn off(&mut self) {
        self.playing = None;
    }

    pub fn duration_ms_to_samples(&self, millis: u32) -> Duration {
        Duration::from_millis(self.sample_rate, millis)
    }

    pub fn next(&mut self) -> Sample {
        self.sample_count += 1;
        let (done, sample) = if let Some(ref mut playing) = &mut self.playing {
            let offset = playing.samples_played % playing.cycle_length;
            let sample = if playing.samples_high == 0 {
                Sample(0)
            } else if offset < playing.samples_high {
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
