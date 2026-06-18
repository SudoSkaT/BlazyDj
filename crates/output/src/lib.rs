use std::sync::{Arc, Mutex};

pub trait AudioOutput: Send + Sync {
    fn start(
        &self,
        sample_rate: u32,
        channels: u16,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn stop(&self);
    fn write_samples(&self, samples: &[f32]);
}

#[derive(Clone)]
pub struct MockOutput {
    buffer: Arc<Mutex<Vec<f32>>>,
    started: Arc<Mutex<bool>>,
    sample_rate: Arc<Mutex<u32>>,
    channels: Arc<Mutex<u16>>,
}

impl MockOutput {
    pub fn new() -> Self {
        MockOutput {
            buffer: Arc::new(Mutex::new(Vec::new())),
            started: Arc::new(Mutex::new(false)),
            sample_rate: Arc::new(Mutex::new(0)),
            channels: Arc::new(Mutex::new(0)),
        }
    }

    pub fn buffer(&self) -> Vec<f32> {
        match self.buffer.lock() {
            Ok(g) => g.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        }
    }
}

impl AudioOutput for MockOutput {
    fn start(
        &self,
        sample_rate: u32,
        channels: u16,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self.started.lock() {
            Ok(mut s) => *s = true,
            Err(poisoned) => *poisoned.into_inner() = true,
        }
        match self.sample_rate.lock() {
            Ok(mut r) => *r = sample_rate,
            Err(poisoned) => *poisoned.into_inner() = sample_rate,
        }
        match self.channels.lock() {
            Ok(mut c) => *c = channels,
            Err(poisoned) => *poisoned.into_inner() = channels,
        }
        Ok(())
    }

    fn stop(&self) {
        match self.started.lock() {
            Ok(mut s) => *s = false,
            Err(poisoned) => *poisoned.into_inner() = false,
        }
    }

    fn write_samples(&self, samples: &[f32]) {
        match self.buffer.lock() {
            Ok(mut b) => b.extend_from_slice(samples),
            Err(poisoned) => poisoned.into_inner().extend_from_slice(samples),
        }
    }
}

impl Default for MockOutput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_output_receives_samples() {
        let out = MockOutput::new();
        out.start(48000, 2).unwrap();
        out.write_samples(&[0.1, -0.1, 0.2]);
        out.stop();
        let buf = out.buffer();
        assert_eq!(buf.len(), 3);
        assert!((buf[0] - 0.1).abs() < 1e-6);
    }
}
