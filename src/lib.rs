#![no_std]

pub const MAX_VOLUME: u8 = 255;

/// We have a four channel synthesiser.
#[derive(Debug, Copy, Clone)]
pub enum Channel {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
}

pub const CHANNEL_0: Channel = Channel::Channel0;
pub const CHANNEL_1: Channel = Channel::Channel1;
pub const CHANNEL_2: Channel = Channel::Channel2;
pub const CHANNEL_3: Channel = Channel::Channel3;

/// Our synthesiser. You can tell it to play notes, then repeatedly ask it for
/// samples (which are calculated as you ask for them). You could either
/// buffer those samples and dispatch the to a PC sound card, or pass them to
/// a DAC in real-time. The samples are calculated by summing together the
/// outputs of the four channels.
pub struct Synth {
    sample_rate: u32,
    channels: [Oscillator; 4],
}

/// Our oscillator produces one of four waveforms.
#[derive(Debug, Copy, Clone)]
pub enum Waveform {
    Sine,
    Sawtooth,
    Square,
    Noise,
}

/// Our `Synth` has four of these oscillators, all running independently.
struct Oscillator {
    /// Which waveform we're playing.
    waveform: &'static [i8; 256],
    /// Calculated from the note frequency, sets how far we step through the
    /// waveform for each sample.
    phase_step: u16,
    /// Sets our current position within the waveform.
    phase_accumulator: u16,
    /// Controls the volume of this channel relative to the others.
    volume: u8,
    /// If None, note is infinite. It Some(x), x is reduced by one for every
    /// sample played, and when it hits zero, the channel is muted.
    duration: Option<Duration>,
}

/// A single signed 8-bit audio sample.
#[derive(Debug)]
pub struct Sample(i8);

/// The length of a note in sample ticks.
#[derive(Debug)]
pub struct Duration(u32);

/// A frequency in centi-hertz.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Frequency(u32);

/// Notes on an piano keyboard, where A4 = 440 Hz.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Note {
    Rest,
    C0,
    CsDb0,
    D0,
    DsEb0,
    E0,
    F0,
    FsGb0,
    G0,
    GsAb0,
    A0,
    AsBb0,
    B0,
    C1,
    CsDb1,
    D1,
    DsEb1,
    E1,
    F1,
    FsGb1,
    G1,
    GsAb1,
    A1,
    AsBb1,
    B1,
    C2,
    CsDb2,
    D2,
    DsEb2,
    E2,
    F2,
    FsGb2,
    G2,
    GsAb2,
    A2,
    AsBb2,
    B2,
    C3,
    CsDb3,
    D3,
    DsEb3,
    E3,
    F3,
    FsGb3,
    G3,
    GsAb3,
    A3,
    AsBb3,
    B3,
    C4,
    CsDb4,
    D4,
    DsEb4,
    E4,
    F4,
    FsGb4,
    G4,
    GsAb4,
    A4,
    AsBb4,
    B4,
    C5,
    CsDb5,
    D5,
    DsEb5,
    E5,
    F5,
    FsGb5,
    G5,
    GsAb5,
    A5,
    AsBb5,
    B5,
    C6,
    CsDb6,
    D6,
    DsEb6,
    E6,
    F6,
    FsGb6,
    G6,
    GsAb6,
    A6,
    AsBb6,
    B6,
    C7,
    CsDb7,
    D7,
    DsEb7,
    E7,
    F7,
    FsGb7,
    G7,
    GsAb7,
    A7,
    AsBb7,
    B7,
    C8,
    CsDb8,
    D8,
    DsEb8,
    E8,
    F8,
    FsGb8,
    G8,
    GsAb8,
    A8,
    AsBb8,
    B8,
}

/// A complete sine wave sampled as 256 signed 8-bit values.
static SINE_256: [i8; 256] = [
    0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 59, 62, 65, 67, 70,
    73, 75, 78, 80, 82, 85, 87, 89, 91, 94, 96, 98, 100, 102, 103, 105, 107, 108, 110, 112, 113,
    114, 116, 117, 118, 119, 120, 121, 122, 123, 123, 124, 125, 125, 126, 126, 126, 126, 126, 127,
    126, 126, 126, 126, 126, 125, 125, 124, 123, 123, 122, 121, 120, 119, 118, 117, 116, 114, 113,
    112, 110, 108, 107, 105, 103, 102, 100, 98, 96, 94, 91, 89, 87, 85, 82, 80, 78, 75, 73, 70, 67,
    65, 62, 59, 57, 54, 51, 48, 45, 42, 39, 36, 33, 30, 27, 24, 21, 18, 15, 12, 9, 6, 3, 0, -3, -6,
    -9, -12, -15, -18, -21, -24, -27, -30, -33, -36, -39, -42, -45, -48, -51, -54, -57, -59, -62,
    -65, -67, -70, -73, -75, -78, -80, -82, -85, -87, -89, -91, -94, -96, -98, -100, -102, -103,
    -105, -107, -108, -110, -112, -113, -114, -116, -117, -118, -119, -120, -121, -122, -123, -123,
    -124, -125, -125, -126, -126, -126, -126, -126, -127, -126, -126, -126, -126, -126, -125, -125,
    -124, -123, -123, -122, -121, -120, -119, -118, -117, -116, -114, -113, -112, -110, -108, -107,
    -105, -103, -102, -100, -98, -96, -94, -91, -89, -87, -85, -82, -80, -78, -75, -73, -70, -67,
    -65, -62, -59, -57, -54, -51, -48, -45, -42, -39, -36, -33, -30, -27, -24, -21, -18, -15, -12,
    -9, -6, -3,
];

/// A square wave sampled as 256 signed 8-bit values.
static SQUARE_256: [i8; 256] = [
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
    127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
    -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127, -127,
];

/// Static white noise sampled as 256 signed 8-bit values.
static NOISE_256: [i8; 256] = [
    -63, -8, -26, -77, -14, 44, -42, 96, 114, -103, 78, 117, -127, 90, 94, 72, 75, -1, 81, -60, 27,
    -90, -115, 49, 126, -123, -84, 89, -42, 84, 73, -123, -51, -90, 77, 105, -65, -57, -52, -128,
    -101, 32, 14, 126, -43, -101, 13, 20, 28, 92, -12, -94, 31, -103, -68, 76, 87, -92, 3, -60, 88,
    -99, 38, 8, 29, 87, -84, 43, -32, 61, -110, 41, 6, 2, -26, -102, -63, 100, 39, 13, -119, -55,
    -99, -84, 110, 5, -11, 58, -95, -35, -116, 115, 25, 78, -127, -5, -111, -58, -121, -128, 9,
    -98, 12, 97, 99, 117, 115, 50, 106, 6, -122, 89, -83, -7, -19, -87, -119, -108, -85, -105, 91,
    -84, -109, 64, -61, 58, -84, 49, -83, -52, 36, -84, 13, 68, 92, -74, 39, -3, 23, -23, 56, 18,
    53, -20, 66, 54, 16, -91, 30, -90, -54, -106, 94, 56, -35, -41, 121, -66, 75, 46, 120, -105,
    -48, -34, 120, 58, -48, 18, -87, -127, -32, -38, 12, 115, -62, 22, 87, 93, -51, 30, 29, -1,
    109, -13, -61, 93, -79, 31, 0, 117, -86, -79, 66, -52, 81, 119, 5, -85, 90, 125, 78, 123, -75,
    -70, -1, 122, -43, -55, 96, -106, 39, 50, -74, -125, -96, 57, -35, -29, 17, 30, 5, -48, -24,
    55, -102, -95, -85, 44, -84, -52, 4, -77, 98, 25, -117, 46, -75, 39, -52, 102, 113, 98, -88,
    -25, -121, 83, -9, 85, -35, 49, -89, -116, 66, -95, 99, -33,
];

/// A sawtooth wave sampled as 256 signed 8-bit values.
static SAWTOOTH_256: [i8; 256] = [
    -127, -127, -126, -125, -124, -123, -122, -121, -120, -119, -118, -117, -116, -115, -114, -113,
    -112, -111, -110, -109, -108, -107, -106, -105, -104, -103, -102, -101, -100, -99, -98, -97,
    -96, -95, -94, -93, -92, -91, -90, -89, -88, -87, -86, -85, -84, -83, -82, -81, -80, -79, -78,
    -77, -76, -75, -74, -73, -72, -71, -70, -69, -68, -67, -66, -65, -64, -63, -62, -61, -60, -59,
    -58, -57, -56, -55, -54, -53, -52, -51, -50, -49, -48, -47, -46, -45, -44, -43, -42, -41, -40,
    -39, -38, -37, -36, -35, -34, -33, -32, -31, -30, -29, -28, -27, -26, -25, -24, -23, -22, -21,
    -20, -19, -18, -17, -16, -15, -14, -13, -12, -11, -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0,
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
    75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98,
    99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
    118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
];

impl Synth {
    pub fn new(sample_rate: u32) -> Synth {
        Synth {
            sample_rate,
            channels: [
                Oscillator {
                    phase_accumulator: 0,
                    phase_step: 0,
                    volume: 0,
                    duration: None,
                    waveform: &SINE_256,
                },
                Oscillator {
                    phase_accumulator: 0,
                    phase_step: 0,
                    volume: 0,
                    duration: None,
                    waveform: &SINE_256,
                },
                Oscillator {
                    phase_accumulator: 0,
                    phase_step: 0,
                    volume: 0,
                    duration: None,
                    waveform: &SINE_256,
                },
                Oscillator {
                    phase_accumulator: 0,
                    phase_step: 0,
                    volume: 0,
                    duration: None,
                    waveform: &SINE_256,
                },
            ],
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn play<T>(
        &mut self,
        channel: Channel,
        note: T,
        duration: Option<Duration>,
        volume: u8,
        waveform: Waveform,
    ) where T: Into<Frequency> {
        let step = self.frequency_to_phase_step(note.into());
        let ch = &mut self.channels[channel as usize];
        ch.phase_accumulator = 0;
        ch.phase_step = step;
        ch.volume = volume;
        ch.duration = duration;
        ch.waveform = match waveform {
            Waveform::Sine => &SINE_256,
            Waveform::Noise => &NOISE_256,
            Waveform::Sawtooth => &SAWTOOTH_256,
            Waveform::Square => &SQUARE_256,
        };
    }

    pub fn off(&mut self, channel: Channel) {
        let ch = &mut self.channels[channel as usize];
        ch.volume = 0;
        ch.phase_accumulator = 0;
        ch.phase_step = 0;
        ch.duration = None;
    }

    pub fn duration_ms_to_samples(&self, millis: u32) -> Duration {
        Duration::from_millis(self.sample_rate, millis)
    }

    pub fn next(&mut self) -> Sample {
        let mut accu: i32 = 0;
        for osc in &mut self.channels {
            osc.phase_accumulator = osc.phase_accumulator.wrapping_add(osc.phase_step);
            let offset = osc.phase_accumulator >> 8;
            let hi_res_sample = (osc.waveform[offset as usize] as i32) * (osc.volume as i32);
            accu += hi_res_sample;
            let silence = if let Some(ref mut duration) = osc.duration {
                if duration.0 > 0 {
                    duration.0 -= 1;
                }
                if duration.0 == 0 {
                    true
                } else {
                    false
                }
            } else {
                false
            };
            if silence {
                osc.volume = 0;
            }
        }
        Self::downmix(accu)
    }

    /// Our waveforms are 256 samples long. This routine converts a playback
    /// frequency into an amount we increment our phase accumulator every
    /// playback sample. The result is a 16-bit fixed-point value (8 bits
    /// integer + 8 bits fraction). That is, we divide accumulated phase by
    /// 256 to get the integer sample index, and carry the fraction for next
    /// time around.
    ///
    /// To play the waveform at 1 Hz, we need to upscale the 256 samples to
    /// `self.sample_rate` samples, with a phase step of 256 / self.sample_rate.
    ///
    /// To play the waveform at 10 Hz, we need to upscale the 256 samples to
    /// `self.sample_rate` samples, with a phase step of 10 * (256 / self.sample_rate).
    ///
    /// To play the waveform at 1000 Hz, we need to upscale the 256 samples to
    /// `self.sample_rate` samples, with a phase step of 1000 * (256 / self.sample_rate).
    ///
    /// `phase_step = note.hertz() * (256 / self.sample_rate)`
    /// `phase_step_fp = 256 * note.hertz() * (256 / self.sample_rate)`
    /// `phase_step_fp = 256 * note.centi_hertz() * (256 / (100 * self.sample_rate))`
    fn frequency_to_phase_step(&self, frequency: Frequency) -> u16 {
        // This is carefully arrange to try and avoid overflow. We should be
        // good up to around 167.8 kHz (which is far higher than we can play).
        let mut step = frequency.centi_hertz() * 256;
        step /= self.sample_rate;
        step *= 256;
        step /= 100;
        step as u16
    }

    pub fn downmix(hi_res_sample: i32) -> Sample {
        // We summed four 16-bit numbers to give an 18-bit number
        let mut low_res_sample = hi_res_sample >> 10;
        if low_res_sample > 127 {
            low_res_sample = 127;
        }
        if low_res_sample < -128 {
            low_res_sample = 128;
        }
        // low_res_sample now in [-128..=127]
        Sample(low_res_sample as i8)
    }
}

impl Duration {
    pub fn from_millis(sample_rate: u32, millis: u32) -> Duration {
        Duration(sample_rate * millis / 1000)
    }
}

impl core::convert::Into<u8> for Sample {
    fn into(self) -> u8 {
        let mut intermediate: i16 = self.0.into();
        intermediate += 128;
        (intermediate as u8)
    }
}

impl Note {
    pub fn hertz(self) -> f32 {
        (self as u32) as f32 / 100.0
    }

    pub fn centi_hertz(self) -> u32 {
        self as u32
    }
}

impl Frequency {
    pub fn from_hertz(hertz: u16) -> Frequency {
        Frequency(hertz as u32 * 16)
    }

    pub fn from_centi_hertz(centi_hertz: u32) -> Frequency {
        Frequency(centi_hertz)
    }

    pub fn centi_hertz(&self) -> u32 {
        self.0
    }
}

impl core::convert::Into<Frequency> for Note {
    fn into(self) -> Frequency {
        Frequency::from_centi_hertz(match self {
            Note::Rest => 0,
            Note::C0 => 1635,
            Note::CsDb0 => 1732,
            Note::D0 => 1835,
            Note::DsEb0 => 1945,
            Note::E0 => 2060,
            Note::F0 => 2183,
            Note::FsGb0 => 2312,
            Note::G0 => 2450,
            Note::GsAb0 => 2596,
            Note::A0 => 2750,
            Note::AsBb0 => 2914,
            Note::B0 => 3087,
            Note::C1 => 3270,
            Note::CsDb1 => 3465,
            Note::D1 => 3671,
            Note::DsEb1 => 3889,
            Note::E1 => 4120,
            Note::F1 => 4365,
            Note::FsGb1 => 4625,
            Note::G1 => 4900,
            Note::GsAb1 => 5191,
            Note::A1 => 5500,
            Note::AsBb1 => 5827,
            Note::B1 => 6174,
            Note::C2 => 6541,
            Note::CsDb2 => 6930,
            Note::D2 => 7342,
            Note::DsEb2 => 7778,
            Note::E2 => 8241,
            Note::F2 => 8731,
            Note::FsGb2 => 9250,
            Note::G2 => 9800,
            Note::GsAb2 => 10383,
            Note::A2 => 11000,
            Note::AsBb2 => 11654,
            Note::B2 => 12347,
            Note::C3 => 13081,
            Note::CsDb3 => 13859,
            Note::D3 => 14683,
            Note::DsEb3 => 15556,
            Note::E3 => 16481,
            Note::F3 => 17461,
            Note::FsGb3 => 18500,
            Note::G3 => 19600,
            Note::GsAb3 => 20765,
            Note::A3 => 22000,
            Note::AsBb3 => 23308,
            Note::B3 => 24694,
            Note::C4 => 26163,
            Note::CsDb4 => 27718,
            Note::D4 => 29366,
            Note::DsEb4 => 31113,
            Note::E4 => 32963,
            Note::F4 => 34923,
            Note::FsGb4 => 36999,
            Note::G4 => 39200,
            Note::GsAb4 => 41530,
            Note::A4 => 44000,
            Note::AsBb4 => 46616,
            Note::B4 => 49388,
            Note::C5 => 52325,
            Note::CsDb5 => 55437,
            Note::D5 => 58733,
            Note::DsEb5 => 62225,
            Note::E5 => 65925,
            Note::F5 => 69846,
            Note::FsGb5 => 73999,
            Note::G5 => 78399,
            Note::GsAb5 => 83061,
            Note::A5 => 88000,
            Note::AsBb5 => 93233,
            Note::B5 => 98777,
            Note::C6 => 104650,
            Note::CsDb6 => 110873,
            Note::D6 => 117466,
            Note::DsEb6 => 124451,
            Note::E6 => 131851,
            Note::F6 => 139691,
            Note::FsGb6 => 147998,
            Note::G6 => 156798,
            Note::GsAb6 => 166122,
            Note::A6 => 176000,
            Note::AsBb6 => 186466,
            Note::B6 => 197553,
            Note::C7 => 209300,
            Note::CsDb7 => 221746,
            Note::D7 => 234932,
            Note::DsEb7 => 248902,
            Note::E7 => 263702,
            Note::F7 => 279383,
            Note::FsGb7 => 295996,
            Note::G7 => 313596,
            Note::GsAb7 => 332244,
            Note::A7 => 352000,
            Note::AsBb7 => 372931,
            Note::B7 => 395107,
            Note::C8 => 418601,
            Note::CsDb8 => 443492,
            Note::D8 => 469863,
            Note::DsEb8 => 497803,
            Note::E8 => 527404,
            Note::F8 => 558765,
            Note::FsGb8 => 591991,
            Note::G8 => 627193,
            Note::GsAb8 => 664488,
            Note::A8 => 704000,
            Note::AsBb8 => 745862,
            Note::B8 => 790213,
        })
    }
}
