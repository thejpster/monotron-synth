extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
extern crate monotron_synth;

use monotron_synth::{Channel, Note, Synth, Waveform, MAX_VOLUME};
use psimple::Simple;
use pulse::stream::Direction;
use std::io::Write;

const SAMPLE_RATE: u32 = 80_000_000 / 2112;
const FRAME_LENGTH_SAMPLES: usize = SAMPLE_RATE as usize / 60;
const FRAMES_PER_BAR: usize = 128;

#[derive(Debug)]
enum Error {
    AudioError(pulse::error::PAErr),
    IOError(std::io::Error),
}

impl std::convert::From<pulse::error::PAErr> for Error {
    fn from(err: pulse::error::PAErr) -> Error {
        Error::AudioError(err)
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

fn main() -> Result<(), Error> {
    let spec = pulse::sample::Spec {
        format: pulse::sample::Format::U8,
        channels: 1,
        rate: SAMPLE_RATE,
    };
    assert!(spec.is_valid());

    let s = Simple::new(
        None,                // Use the default server
        "MonotronSynthBeep", // Our application's name
        Direction::Playback, // We want a playback stream
        None,                // Use the default device
        "Music",             // Description of our stream
        &spec,               // Our sample format
        None,                // Use default channel map
        None,                // Use default buffering attributes
    )
    .unwrap();

    let mut synth = Synth::new(SAMPLE_RATE);

    let mut output_file = std::fs::File::create("audio.raw")?;

    #[derive(Debug)]
    enum Length {
        Whole,
        Half,
        Quarter,
        DottedQuarter,
        Eighth,
        Sixteenth,
    }

    struct Track<'a> {
        channel: Channel,
        waveform: Waveform,
        volume: u8,
        play_idx: usize,
        play_next_at: usize,
        notes: &'a [(Option<Note>, Length)],
    }

    // --- F4
    // E4
    // --- D4
    // C4
    // --- B3
    // A3
    // --- G3
    // F3
    // --- E3

    let mut track0 = Track {
        channel: Channel::Channel0,
        play_idx: 0,
        play_next_at: 0,
        waveform: Waveform::Square,
        volume: MAX_VOLUME / 2,
        notes: &[
            // Bar 1
            (Some(Note::E5), Length::Quarter),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::D5), Length::Eighth),
            (Some(Note::E5), Length::Sixteenth),
            (Some(Note::D5), Length::Sixteenth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::B4), Length::Eighth),
            // Bar 2
            (Some(Note::A4), Length::Quarter),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::E5), Length::Quarter),
            (Some(Note::D5), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            // Bar 3
            (Some(Note::B4), Length::DottedQuarter),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::D5), Length::Quarter),
            (Some(Note::E5), Length::Quarter),
            // Bar 4
            (Some(Note::C5), Length::Quarter),
            (Some(Note::A4), Length::Quarter),
            (Some(Note::A4), Length::Half),
            // Bar 5
            (Some(Note::D5), Length::Quarter),
            (Some(Note::F5), Length::Eighth),
            (Some(Note::A5), Length::Quarter),
            (Some(Note::G5), Length::Eighth),
            (Some(Note::F5), Length::Eighth),
            // Bar 6
            (Some(Note::E5), Length::DottedQuarter),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::E5), Length::Quarter),
            (Some(Note::D5), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            // Bar 7
            (Some(Note::B4), Length::Quarter),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::D5), Length::Quarter),
            (Some(Note::E5), Length::Quarter),
            // Bar 8
            (Some(Note::C5), Length::Quarter),
            (Some(Note::A4), Length::Quarter),
            (Some(Note::A4), Length::Quarter),
            (None, Length::Quarter),
        ],
    };

    let mut track1 = Track {
        channel: Channel::Channel1,
        play_idx: 0,
        waveform: Waveform::Square,
        volume: MAX_VOLUME / 2,
        play_next_at: 0,
        notes: &[
            (Some(Note::B4), Length::Quarter),
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::B4), Length::Eighth),
            (None, Length::Sixteenth),
            (None, Length::Sixteenth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::GsAb4), Length::Eighth),
            // Bar 2
            (Some(Note::E4), Length::Quarter),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::C5), Length::Quarter),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            // Bar 3
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::B4), Length::Quarter),
            (Some(Note::C4), Length::Quarter),
            // Bar 4
            (Some(Note::A4), Length::Quarter),
            (Some(Note::E4), Length::Quarter),
            (Some(Note::E4), Length::Half),
            // Bar 5
            (Some(Note::F4), Length::Quarter),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::C5), Length::Sixteenth),
            (Some(Note::C5), Length::Sixteenth),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            // Bar 6
            (Some(Note::G4), Length::DottedQuarter),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::G4), Length::Eighth),
            (Some(Note::A4), Length::Sixteenth),
            (Some(Note::G4), Length::Sixteenth),
            (Some(Note::F4), Length::Eighth),
            (Some(Note::E4), Length::Eighth),
            // Bar 7
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::GsAb4), Length::Eighth),
            (Some(Note::A4), Length::Eighth),
            (Some(Note::B4), Length::Eighth),
            (Some(Note::G4), Length::Eighth),
            (Some(Note::C5), Length::Eighth),
            (Some(Note::G4), Length::Eighth),
            // Bar 8
            (Some(Note::A4), Length::Eighth),
            (Some(Note::E4), Length::Eighth),
            (Some(Note::E4), Length::Quarter),
            (Some(Note::E4), Length::Quarter),
            (None, Length::Quarter),
        ],
    };

    let mut track2 = Track {
        channel: Channel::Channel2,
        play_idx: 0,
        waveform: Waveform::Sawtooth,
        volume: MAX_VOLUME,
        play_next_at: 0,
        notes: &[
            // Bar 1
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            // Bar 2
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            // Bar 3
            (Some(Note::GsAb2), Length::Eighth),
            (Some(Note::GsAb3), Length::Eighth),
            (Some(Note::GsAb2), Length::Eighth),
            (Some(Note::GsAb3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::E2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            // Bar 4
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::A3), Length::Eighth),
            // Bar 5
            (Some(Note::D3), Length::Eighth),
            (Some(Note::D2), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::D2), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::D2), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::F2), Length::Eighth),
            // Bar 6
            (Some(Note::C2), Length::Eighth),
            (Some(Note::C3), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::C3), Length::Eighth),
            (Some(Note::C2), Length::Eighth),
            (Some(Note::G2), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::G2), Length::Eighth),
            // Bar 7
            (Some(Note::B2), Length::Eighth),
            (Some(Note::B3), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::B3), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (None, Length::Eighth),
            (Some(Note::GsAb3), Length::Eighth),
            // Bar 8
            (Some(Note::A2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::A2), Length::Eighth),
            (Some(Note::E3), Length::Eighth),
            (Some(Note::A2), Length::Quarter),
            (None, Length::Quarter),
        ],
    };

    let mut frame_count = 0;
    loop {
        let mut again = true;
        while again {
            again = false;
            for track in &mut [&mut track0, &mut track1, &mut track2] {
                let (note, length) = &track.notes[track.play_idx];
                if frame_count == track.play_next_at {
                    if let Some(pitch) = note {
                        println!(
                            "{:?} {:?} @ {} for {:?}",
                            track.channel, pitch, track.play_next_at, length
                        );
                        synth.play(track.channel, *pitch, track.volume, track.waveform);
                    } else {
                        synth.off(track.channel);
                    }
                    track.play_next_at += match length {
                        Length::Whole => FRAMES_PER_BAR,
                        Length::Half => FRAMES_PER_BAR / 2,
                        Length::Quarter => FRAMES_PER_BAR / 4,
                        Length::DottedQuarter => 3 * (FRAMES_PER_BAR / 8),
                        Length::Eighth => FRAMES_PER_BAR / 8,
                        Length::Sixteenth => FRAMES_PER_BAR / 16,
                    };
                    track.play_idx += 1;
                    if track.play_idx >= track.notes.len() {
                        track.play_idx = 0;
                    }
                    again = true;
                }
            }
        }
        frame_count += 1;
        println!("{}", frame_count);
        // Play a frame
        let mut samples = [0; FRAME_LENGTH_SAMPLES];
        for sample in samples.iter_mut() {
            let s = synth.next();
            *sample = s.into();
            if *sample == 0 || *sample == 255 {
                print!("Clip!");
            }
        }
        output_file.write_all(&samples)?;
        s.write(&samples)?;
    }
}
