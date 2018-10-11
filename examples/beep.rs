extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
extern crate monotron_synth;

use monotron_synth::{Channel, Note, Synth, Waveform, MAX_VOLUME};
use psimple::Simple;
use pulse::stream::Direction;

const SAMPLE_RATE: u32 = 80_000_000 / 2112;
const FRAME_LENGTH_SAMPLES: usize = SAMPLE_RATE as usize / 60;

fn main() -> Result<(), pulse::error::PAErr> {
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

    let notes = [
        (Channel::Channel0, Note::C4, 0, MAX_VOLUME),
        (Channel::Channel1, Note::E4, 30, MAX_VOLUME),
        (Channel::Channel2, Note::G4, 60, MAX_VOLUME),
        (Channel::Channel2, Note::G4, 90, 0),
        (Channel::Channel1, Note::E4, 120, 0),
    ];

    const WAVEFORM: Waveform = Waveform::Sawtooth;
    let mut play_idx = 0;
    let mut frame_count = 0;
    loop {
        let (channel, note, start_frame, volume) = notes[play_idx];
        if frame_count == start_frame {
            println!("{:?} {:?} @ {}", channel, note, start_frame);
            synth.play(channel, note, volume, WAVEFORM);
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
            s.write(&samples)?;
        }
    }
}
