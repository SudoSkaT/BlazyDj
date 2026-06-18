#![cfg(feature = "cpal-support")]

use crate::AudioOutput;
use ringbuf::RingBuffer;

// This module is feature-gated (cpal-support). It provides a CPAL-based AudioOutput
// implementation that uses a ring buffer to pass mixed samples to the audio callback.

use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub struct CpalOutput {
    // Producer is used by write_samples(); wrapped in Mutex for &self access
    prod: Mutex<ringbuf::Producer<f32>>,
    // Signal to stop the background thread that owns the stream
    stop_flag: Arc<AtomicBool>,
    // Optional handle to the background thread
    thread_handle: Mutex<Option<std::thread::JoinHandle<()>>>,
}

#[allow(dead_code)]
impl CpalOutput {
    pub fn new() -> Self {
        // default capacity: 64k samples
        let rb = RingBuffer::<f32>::new(64 * 1024);
        let (prod, _cons) = rb.split();
        let prod = Mutex::new(prod);
        Self {
            prod,
            stop_flag: Arc::new(AtomicBool::new(false)),
            thread_handle: Mutex::new(None),
        }
    }
}

impl AudioOutput for CpalOutput {
    fn start(
        &self,
        _sample_rate: u32,
        _channels: u16,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Spawn a background thread that will (when fully implemented) create a cpal stream
        // and move the consumer into the audio callback. For this POC, keep the thread alive
        // and periodically check the stop flag so the stream can be dropped when stop() is called.
        let stop = self.stop_flag.clone();
        let handle = thread::spawn(move || {
            // In a full implementation: create cpal device, config, and start stream with consumer in callback.
            while !stop.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(50));
            }
        });
        let mut guard = self.thread_handle.lock().unwrap();
        *guard = Some(handle);
        Ok(())
    }

    fn stop(&self) {
        self.stop_flag.store(true, Ordering::SeqCst);
        if let Some(h) = self.thread_handle.lock().unwrap().take() {
            let _ = h.join();
        }
    }

    fn write_samples(&self, samples: &[f32]) {
        if let Ok(mut p) = self.prod.lock() {
            let _ = p.push_slice(samples);
        }
    }
}
