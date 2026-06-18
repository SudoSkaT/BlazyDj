//! Waveform generation utilities.

/// Generate a waveform of target_len samples from input samples by computing the
/// peak absolute amplitude per block and normalizing to [0.0, 1.0].
#[allow(clippy::manual_div_ceil, clippy::collapsible_if)]
pub fn generate_waveform(samples: &[f32], target_len: usize) -> Vec<f32> {
    if target_len == 0 || samples.is_empty() {
        return vec![0.0; target_len];
    }
    let block_size = (samples.len() + target_len - 1) / target_len; // ceil
    let mut out = Vec::with_capacity(target_len);
    for i in 0..target_len {
        let start = i * block_size;
        let end = ((i + 1) * block_size).min(samples.len());
        if start >= end {
            out.push(0.0);
            continue;
        }
        let mut peak = 0.0_f32;
        for s in &samples[start..end] {
            let a = s.abs();
            if a > peak {
                peak = a;
            }
        }
        out.push(peak);
    }
    // normalize
    if let Some(max) = out
        .iter()
        .cloned()
        .fold(None::<f32>, |acc, v| Some(acc.map_or(v, |m| m.max(v))))
    {
        if max > 0.0 {
            for v in out.iter_mut() {
                *v /= max;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waveform_basic() {
        let samples = vec![0.0, 1.0, -1.0, 0.5, -0.5, 0.0];
        let wf = generate_waveform(&samples, 3);
        assert_eq!(wf.len(), 3);
        assert!((wf[0] - 1.0).abs() < 1e-6);
    }
}
