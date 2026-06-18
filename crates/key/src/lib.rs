//! Simple key detection crate for Phase 9.
//!
//! Provides a background-friendly, dependency-free key estimator using a
//! naive DFT per semitone to build a chroma vector and the
//! Krumhansl-Schmuckler key-finding algorithm.

use std::f32::consts::PI;

const MIDI_LOW: i32 = 36; // C2
const MIDI_HIGH: i32 = 96; // C7

const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

// Krumhansl major and minor key profiles
const KRUMHANSL_MAJOR: [f32; 12] = [
    6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88,
];

const KRUMHANSL_MINOR: [f32; 12] = [
    6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17,
];

/// Estimate the musical key (tonic and mode) for a buffer of mono samples.
/// Returns (tonic_name, mode) where mode is "major" or "minor".
///
/// This is intended for background analysis, not audio-thread usage.
pub fn detect_key(samples: &[f32], sample_rate: usize) -> Option<(String, String)> {
    if samples.is_empty() || sample_rate == 0 {
        return None;
    }

    // Limit processing to the first few seconds to keep computation bounded.
    let max_secs = 4usize;
    let n = samples.len().min(sample_rate * max_secs);
    if n < 64 {
        return None;
    }

    let slice = &samples[..n];

    // Apply Hann window
    let mut windowed = Vec::with_capacity(n);
    for (i, &s) in slice.iter().enumerate() {
        let w = 0.5f32 * (1.0f32 - (2.0f32 * PI * i as f32 / (n as f32 - 1.0f32)).cos());
        windowed.push(s * w);
    }

    // Per-semitone energies
    let mut semitone_energy = vec![0.0f32; (MIDI_HIGH - MIDI_LOW + 1) as usize];

    for (m_idx, midi) in (MIDI_LOW..=MIDI_HIGH).enumerate() {
        let freq = midi_to_freq(midi);
        let ang = 2.0 * PI * freq / sample_rate as f32;
        let mut re = 0.0f32;
        let mut im = 0.0f32;
        for (i, &s) in windowed.iter().enumerate() {
            let a = ang * i as f32;
            re += s * a.cos();
            im += s * a.sin();
        }
        semitone_energy[m_idx] = re * re + im * im;
    }

    // Build chroma vector (12 pitch classes)
    let mut chroma = vec![0.0f32; 12];
    for (i, &m_energy) in semitone_energy.iter().enumerate() {
        let midi = MIDI_LOW + i as i32;
        let pc = (midi.rem_euclid(12) as usize) % 12;
        chroma[pc] += m_energy;
    }

    let sum: f32 = chroma.iter().copied().sum();
    if sum <= 0.0 {
        return None;
    }

    // Normalize chroma
    for v in chroma.iter_mut() {
        *v /= sum;
    }

    // Find best key by correlating with rotated major/minor profiles
    let mut best_score = f32::NEG_INFINITY;
    let mut best_tonic = 0usize;
    let mut best_mode = "major";

    for tonic in 0..12 {
        // major
        let score_major = dot(&chroma, &rotated_profile(&KRUMHANSL_MAJOR, tonic));
        if score_major > best_score {
            best_score = score_major;
            best_tonic = tonic;
            best_mode = "major";
        }
        // minor
        let score_minor = dot(&chroma, &rotated_profile(&KRUMHANSL_MINOR, tonic));
        if score_minor > best_score {
            best_score = score_minor;
            best_tonic = tonic;
            best_mode = "minor";
        }
    }

    Some((NOTE_NAMES[best_tonic].to_string(), best_mode.to_string()))
}

fn midi_to_freq(midi: i32) -> f32 {
    440.0f32 * 2f32.powf((midi as f32 - 69.0) / 12.0)
}

fn rotated_profile(profile: &[f32; 12], shift: usize) -> [f32; 12] {
    let mut out = [0.0f32; 12];
    for i in 0..12 {
        out[i] = profile[(i + 12 - shift) % 12];
    }
    out
}

fn dot(a: &[f32], b: &[f32; 12]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_a_tonic_from_sine() {
        let sample_rate = 44100usize;
        let freq = 440.0f32; // A4
        let duration = 2.0f32;
        let n = (sample_rate as f32 * duration) as usize;
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = i as f32 / sample_rate as f32;
            samples.push((2.0 * PI * freq * t).sin());
        }
        let res = detect_key(&samples, sample_rate).expect("should detect key");
        assert_eq!(res.0, "A");
    }
}
