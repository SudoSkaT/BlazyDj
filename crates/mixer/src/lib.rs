/// Basic mixing utilities for Phase 4 Mixer Engine.
///
/// Provides simple sample mixing and a minimal Deck state for tests.
#[derive(Debug, Clone)]
pub struct Deck {
    pub gain: f32,
    pub position: usize,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            gain: 1.0,
            position: 0,
        }
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

pub fn mix_samples(a: &[f32], b: &[f32], crossfade: f32, gain_a: f32, gain_b: f32) -> Vec<f32> {
    let len = std::cmp::min(a.len(), b.len());
    let mut out = Vec::with_capacity(len);
    let cf = if crossfade.is_nan() {
        0.0
    } else {
        crossfade.clamp(0.0, 1.0)
    };
    for i in 0..len {
        let sa = a[i] * gain_a * (1.0 - cf);
        let sb = b[i] * gain_b * cf;
        out.push(sa + sb);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_crossfade_edges() {
        let a = vec![1.0_f32, 0.0, -1.0];
        let b = vec![0.0_f32, 1.0, 0.5];

        let out_a = mix_samples(&a, &b, 0.0, 1.0, 1.0);
        assert_eq!(out_a, a);

        let out_b = mix_samples(&a, &b, 1.0, 1.0, 1.0);
        assert_eq!(out_b, b);

        let out_mid = mix_samples(&a, &b, 0.5, 1.0, 1.0);
        assert_eq!(out_mid.len(), 3);
        assert!((out_mid[0] - 0.5_f32).abs() < 1e-6);
        assert!((out_mid[1] - 0.5_f32).abs() < 1e-6);
        assert!((out_mid[2] + 0.25_f32).abs() < 1e-6);
    }
}
