//! Simple BPM estimator used for Phase 7 (BPM analysis).
//!
//! This crate provides a lightweight, background-worker estimator. It is
//! intentionally dependency-free and not designed for audio-callback usage.

/// Estimate the BPM of a mono sample buffer using an autocorrelation on a
/// downsampled amplitude envelope. Returns `None` when estimation is not
/// possible.
pub fn estimate_bpm(samples: &[f32], sample_rate: usize) -> Option<f32> {
    if samples.is_empty() || sample_rate == 0 {
        return None;
    }

    // Downsample envelope to keep the autocorrelation fast and robust.
    let target_rate = 200usize; // envelope samples per second
    let hop = ((sample_rate + target_rate / 2) / target_rate).max(1);

    // Build envelope (mean absolute value per hop)
    let envelope: Vec<f32> = samples
        .chunks(hop)
        .map(|chunk| chunk.iter().map(|s| s.abs()).sum::<f32>() / chunk.len() as f32)
        .collect();

    if envelope.len() < 3 {
        return None;
    }

    // Remove DC
    let mean = envelope.iter().copied().sum::<f32>() / envelope.len() as f32;
    let detrended: Vec<f32> = envelope.into_iter().map(|v| v - mean).collect();

    // BPM search range (common dance tempos). Adjust as needed.
    let bpm_min = 60.0f32;
    let bpm_max = 180.0f32;

    let lag_min = ((target_rate as f32) * (60.0f32 / bpm_max)).max(1.0) as usize;
    let lag_max_f = ((target_rate as f32) * (60.0f32 / bpm_min)).min((detrended.len() - 1) as f32);
    if lag_max_f < lag_min as f32 {
        return None;
    }
    let lag_max = lag_max_f as usize;

    // Autocorrelation search for best lag
    let mut best_lag = lag_min;
    let mut best_score = f32::NEG_INFINITY;
    for lag in lag_min..=lag_max {
        let score: f32 = detrended
            .iter()
            .zip(detrended.iter().skip(lag))
            .map(|(a, b)| a * b)
            .sum();
        if score > best_score {
            best_score = score;
            best_lag = lag;
        }
    }

    if best_score <= 0.0 {
        return None;
    }

    let bpm = 60.0f32 * (target_rate as f32) / (best_lag as f32);
    Some(bpm)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Create a synthetic click track at a known BPM and ensure estimator
    // returns a value close to the source tempo.
    #[test]
    fn detect_click_bpm() {
        let sample_rate = 44_100usize;
        let bpm = 120.0f32;
        let duration_secs = 8.0f32;
        let samples_len = (duration_secs * sample_rate as f32) as usize;
        let mut samples = vec![0.0f32; samples_len];

        let interval = ((60.0f32 / bpm) * sample_rate as f32) as usize;
        let mut pos = 0usize;
        while pos < samples_len {
            // simple impulse
            samples[pos] = 1.0;
            if pos + 1 < samples_len {
                samples[pos + 1] = 0.5;
            }
            pos = pos.saturating_add(interval);
        }

        let est = estimate_bpm(&samples, sample_rate).expect("expected detection");
        // allow some tolerance because of downsampling and autocorrelation
        assert!(
            (est - bpm).abs() < 4.0,
            "expected ~{} BPM, got {}",
            bpm,
            est
        );
    }
}
