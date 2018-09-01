extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
extern crate monotron_synth;

use monotron_synth::{Note, Synth};
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
        (Note::C4, 250),
        (Note::G4, 125),
        (Note::Rest, 125),
        (Note::A4, 125),
        (Note::Rest, 125),
        (Note::F4, 500),
    ];
    let mut notes_iter = notes.iter().cycle();

    loop {
        let (note, duration_ms) = *notes_iter.next().unwrap();
        let duration_samples = synth.duration_ms_to_samples(duration_ms);
        synth.play(note, duration_samples);
        while synth.is_playing() {
            let mut samples = [0; FRAME_LENGTH_SAMPLES];
            for sample in samples.iter_mut() {
                *sample = synth.next().into();
            }
            s.write(&samples)?;
        }
    }
}

