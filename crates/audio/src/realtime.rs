use ringbuf::RingBuffer;

/// Simple SPSC ring buffer wrapper for real-time audio paths.
/// Producer writes mixed samples; consumer is intended to be used from audio callback.
#[allow(dead_code)]
pub struct RealtimeRing {
    prod: ringbuf::Producer<f32>,
    cons: ringbuf::Consumer<f32>,
}

#[allow(dead_code)]
impl RealtimeRing {
    /// Create a ring buffer with `capacity` samples (must be > 0).
    pub fn new(capacity: usize) -> Self {
        let rb = RingBuffer::<f32>::new(capacity);
        let (prod, cons) = rb.split();
        Self { prod, cons }
    }

    /// Push as many samples as will fit. Returns number pushed.
    pub fn push_samples(&mut self, samples: &[f32]) -> usize {
        self.prod.push_slice(samples)
    }

    /// Fill the provided output buffer from the ring. Returns number popped.
    pub fn fill_callback(&mut self, out: &mut [f32]) -> usize {
        self.cons.pop_slice(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn ring_push_pop() {
        let mut ring = RealtimeRing::new(1024);
        let data: Vec<f32> = (0..256).map(|i| i as f32).collect();
        let pushed = ring.push_samples(&data);
        assert_eq!(pushed, 256);

        let mut out = vec![0.0_f32; 128];
        let popped = ring.fill_callback(&mut out);
        assert_eq!(popped, 128);
        assert_eq!(out[0], 0.0);

        // push more concurrently
        let ring2 = RealtimeRing::new(512);
        let handle = thread::spawn(move || {
            let mut r = ring2;
            let d: Vec<f32> = (0..200).map(|i| i as f32).collect();
            r.push_samples(&d)
        });
        let pushed2 = handle.join().unwrap();
        assert_eq!(pushed2, 200);
    }
}
