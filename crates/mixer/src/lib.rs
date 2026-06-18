/// Basic mixing utilities for Phase 4 Mixer Engine.
///
/// Provides simple sample mixing and Deck/Mixer types.
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

/// Mix two equal-length sample buffers with a crossfade and per-deck gains.
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

/// High-level mixer managing two decks and a crossfader.
#[derive(Debug, Clone)]
pub struct Mixer {
    pub deck_a: Deck,
    pub deck_b: Deck,
    pub crossfade: f32, // 0.0 => full A, 1.0 => full B
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            deck_a: Deck::default(),
            deck_b: Deck::default(),
            crossfade: 0.0,
        }
    }

    pub fn set_crossfade(&mut self, v: f32) {
        self.crossfade = v.clamp(0.0, 1.0);
    }

    pub fn mix(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        mix_samples(a, b, self.crossfade, self.deck_a.gain, self.deck_b.gain)
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixer_crossfade() {
        let a = vec![1.0_f32, 0.0, -1.0];
        let b = vec![0.0_f32, 1.0, 0.5];

        let mut m = Mixer::new();
        m.set_crossfade(0.0);
        let out_a = m.mix(&a, &b);
        assert_eq!(out_a, a);

        m.set_crossfade(1.0);
        let out_b = m.mix(&a, &b);
        assert_eq!(out_b, b);

        m.set_crossfade(0.5);
        let out_mid = m.mix(&a, &b);
        assert_eq!(out_mid.len(), 3);
        assert!((out_mid[0] - 0.5_f32).abs() < 1e-6);
        assert!((out_mid[1] - 0.5_f32).abs() < 1e-6);
        assert!((out_mid[2] + 0.25_f32).abs() < 1e-6);
    }
}
