extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
extern crate monotron_synth;

use std::io::Write;
use monotron_synth::{Channel, Note, Synth, Waveform, MAX_VOLUME};
use psimple::Simple;
use pulse::stream::Direction;

const SAMPLE_RATE: u32 = 80_000_000 / 2112;
const FRAME_LENGTH_SAMPLES: usize = SAMPLE_RATE as usize / 60;

#[derive(Debug)]
enum Error {
    AudioError(pulse::error::PAErr),
    IOError(std::io::Error)
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

    let notes = [
        (Channel::Channel0, 0, Some((Note::C2, MAX_VOLUME, Waveform::Sawtooth))),
        (Channel::Channel0, 15, None),
        (Channel::Channel0, 30, Some((Note::C2, MAX_VOLUME, Waveform::Sawtooth))),
        (Channel::Channel1, 30, Some((Note::C3, MAX_VOLUME, Waveform::Noise))),
        (Channel::Channel1, 33, None),
        (Channel::Channel0, 45, None),
        (Channel::Channel0, 60, Some((Note::G2, MAX_VOLUME, Waveform::Sawtooth))),
        (Channel::Channel0, 75, None),
        (Channel::Channel0, 90, Some((Note::G2, MAX_VOLUME, Waveform::Sawtooth))),
        (Channel::Channel1, 90, Some((Note::C3, MAX_VOLUME, Waveform::Noise))),
        (Channel::Channel1, 93, None),
        (Channel::Channel0, 105, None),
        (Channel::Channel1, 120, None),
    ];

    let mut play_idx = 0;
    let mut frame_count = 0;
    loop {
        let (channel, start_frame, event) = notes[play_idx];
        if frame_count == start_frame {
            if let Some((note, volume, waveform)) = event {
                println!("{:?} {:?} @ {} in {:?}", channel, note, start_frame, waveform);
                synth.play(channel, note, None, volume, waveform);
            } else {
                synth.off(channel);
            }
            play_idx += 1;
            if play_idx >= notes.len() {
                play_idx = 0;
                frame_count = 0;
            }
        } else {
            println!("{}", frame_count);
            frame_count += 1;
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
}
