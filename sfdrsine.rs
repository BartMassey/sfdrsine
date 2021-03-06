//! Find a low-SFDR 14-bit sine wave.

use std::f64::consts::TAU;

use rayon::prelude::*;
use ordered_float::OrderedFloat;

/// Calculate the SFDR of the given `samples` in dB relative
/// to a sine wave at frequency 1/16 with the given `gain` and
/// `phase` (in radians).
fn sfdr(samples: &[i16;16], gain: f64, phase: f64) -> f64 {
    let error = samples
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let d = s as f64 - gain * f64::sin(TAU / 16.0 * i as f64 + phase);
            d * d
        })
        .sum::<f64>()
        .sqrt();
    20.0 * f64::log10(error / (0.5 * f64::sqrt(2.0) * gain))
}

#[test]
fn test_sfdr() {
    let eg = [
        0, 3107, 5741, 7501, 8119, 7501, 5741, 3107,
        0, -3107, -5741, -7501, -8119, -7501, -5741, -3107,
    ];
    // Simple sanity check: answer should be slightly less
    // than -103 dB.
    assert!(sfdr(&eg, 8119.0, 0.0) < -100.0)
}

/// Generate 16 samples of a sine wave with the given gain
/// and phase.
fn gen_sine(gain: f64, phase: f64) -> [i16;16] {
    let mut samples = [0; 16];
    for (i, s) in samples.iter_mut().enumerate() {
        let y = gain * f64::sin(TAU / 16.0 * i as f64 + phase);
        *s = y.round() as i16;
    }
    samples
}

fn main() {
    // Strategy: generate and test with various gains and
    // phases. Question: Is there a theorem that says that
    // the phase should be 0? It certainly appears that way
    // in practice.
    let gain_steps = 4096 * 256;
    let phase_steps = 256;
    let xs = (0..=gain_steps)
        .into_par_iter()
        .flat_map(move |gi| {
            let gain = 4096.0 + 4096.0 * gi as f64 / gain_steps as f64;
            (0..phase_steps)
                .into_par_iter()
                .map(move |pi| {
                    let phase = TAU / 32.0 * pi as f64 / phase_steps as f64;
                    (gain, phase)
                })
        });
    let (sine_sfdr, sine, gain, phase) = xs
        .map(|(gain, phase)| {
            let sine = gen_sine(gain, phase);
            let sine_sfdr = sfdr(&sine, gain, phase);
            (sine_sfdr, sine, gain, phase)
        })
        .min_by_key(|&(sine_sfdr, _, _, _)| OrderedFloat(sine_sfdr))
        .unwrap();
    println!("{sine_sfdr}, {gain}, {phase}");
    println!("{:#?}", sine);
}
