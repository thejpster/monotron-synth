extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
extern crate monotron_synth;

use monotron_synth::{Channel, Note, Synth, Waveform, MAX_VOLUME};
use psimple::Simple;
use pulse::stream::Direction;
use std::io::Write;

const SAMPLE_RATE: u32 = 80_000_000 / 2112;
const FRAME_LENGTH_SAMPLES: usize = SAMPLE_RATE as usize / 60;

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
    ).unwrap();

    let mut synth = Synth::new(SAMPLE_RATE);

    let mut output_file = std::fs::File::create("audio.raw")?;

    struct Track<'a> {
        channel: Channel,
        play_idx: usize,
        max_frames: usize,
        notes: &'a [(usize, Option<(Note, u8, Waveform)>)],
    }

    // Bass line
    let mut track0 = Track {
        channel: Channel::Channel0,
        play_idx: 0,
        max_frames: 120,
        notes: &[
            (0, Some((Note::C2, MAX_VOLUME, Waveform::Sawtooth))),
            (5, None),
            (15, Some((Note::C2, MAX_VOLUME, Waveform::Sawtooth))),
            (20, None),
            (30, Some((Note::C2, MAX_VOLUME, Waveform::Sawtooth))),
            (35, None),
            (45, Some((Note::C2, MAX_VOLUME, Waveform::Sawtooth))),
            (50, None),
            (60, Some((Note::G2, MAX_VOLUME, Waveform::Sawtooth))),
            (65, None),
            (75, Some((Note::G2, MAX_VOLUME, Waveform::Sawtooth))),
            (80, None),
            (90, Some((Note::G2, MAX_VOLUME, Waveform::Sawtooth))),
            (95, None),
            (105, Some((Note::G2, MAX_VOLUME, Waveform::Sawtooth))),
            (110, None),
        ],
    };

    // Hi-hat
    let mut track1 = Track {
        channel: Channel::Channel1,
        play_idx: 0,
        max_frames: 120,
        notes: &[
            (30, Some((Note::C3, MAX_VOLUME, Waveform::Noise))),
            (33, None),
            (90, Some((Note::C3, MAX_VOLUME, Waveform::Noise))),
            (93, None),
        ],
    };

    // Scale
    let mut track2 = Track {
        channel: Channel::Channel2,
        play_idx: 0,
        max_frames: 120,
        notes: &[
            (0, Some((Note::C4, MAX_VOLUME, Waveform::Sine))),
            (15, Some((Note::D4, MAX_VOLUME, Waveform::Sine))),
            (30, Some((Note::E4, MAX_VOLUME, Waveform::Sine))),
            (45, Some((Note::F4, MAX_VOLUME, Waveform::Sine))),
            (60, Some((Note::G4, MAX_VOLUME, Waveform::Sine))),
            (75, Some((Note::A4, MAX_VOLUME, Waveform::Sine))),
            (90, Some((Note::B4, MAX_VOLUME, Waveform::Sine))),
            (105, Some((Note::C5, MAX_VOLUME, Waveform::Sine))),
        ],
    };

    let mut frame_count = 0;
    loop {
        let mut again = true;
        while again {
            again = false;
            for track in &mut [&mut track0, &mut track1, &mut track2] {
                let (start_frame, event) = track.notes[track.play_idx];
                if (frame_count % track.max_frames) == start_frame {
                    if let Some((note, volume, waveform)) = event {
                        println!(
                            "{:?} {:?} @ {} in {:?}",
                            track.channel, note, start_frame, waveform
                        );
                        synth.play(track.channel, note, volume, waveform);
                    } else {
                        synth.off(track.channel);
                    }
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
