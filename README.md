# Monotron Synth

This is a very rudimentary synthesiser for use on
[Monotron](https://github.com/thejpster/monotron).

It is `#![no_std]` crate with no dynamic memory allocation. It's a small
structure containing four oscillators where each oscillator has a volume, a
frequency and a waveform. You can either modify the oscillators, or pull out
the next sample. Each sample is calculated as the sum of the output of each
oscillator, and the samples are signed 8-bit values.

It produces a pretty gritty noise, which if you're trying to simulate a late
1970s / early 1980s home computer is probably about right.

## Waveforms

* Sine wave
* Triangle wave
* Square wave
* Noise (ish)

## TODO:

* ADSR envelopes

## Licence

MIT or Apache 2 at your choice.
