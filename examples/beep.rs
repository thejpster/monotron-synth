extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
extern crate monotron_synth;

use monotron_synth::{Note, Synth};
use psimple::Simple;
use pulse::stream::Direction;

const SAMPLE_RATE: u32 = 44_100;
const BLOCK_SIZE_10MS: usize = SAMPLE_RATE as usize / 100;

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

    let notes = [Note::C4, Note::G4, Note::A4, Note::F4];
    let mut notes_iter = notes.iter().cycle();

    for _ in 0..20 {
        let d = synth.duration(250);
        synth.play(*notes_iter.next().unwrap(), d);
        for _ in 0..50 {
            let mut samples = [0; BLOCK_SIZE_10MS];
            for sample in samples.iter_mut() {
                *sample = synth.next().into();
            }
            s.write(&samples)?;
        }
    }

    s.drain()?;

    Ok(())
}
