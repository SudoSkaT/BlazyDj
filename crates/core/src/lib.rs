use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    pub sample_rate: u32,
    pub audio_device: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            sample_rate: 48000,
            audio_device: None,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct AppStateHandle(Arc<RwLock<AppState>>);

impl AppStateHandle {
    pub fn new() -> Self {
        AppStateHandle(Arc::new(RwLock::new(AppState::new())))
    }

    pub fn get_sample_rate(&self) -> u32 {
        match self.0.read() {
            Ok(g) => g.sample_rate,
            Err(poisoned) => poisoned.into_inner().sample_rate,
        }
    }

    pub fn set_sample_rate(&self, sr: u32) {
        match self.0.write() {
            Ok(mut g) => g.sample_rate = sr,
            Err(poisoned) => poisoned.into_inner().sample_rate = sr,
        }
    }

    pub fn get_audio_device(&self) -> Option<String> {
        match self.0.read() {
            Ok(g) => g.audio_device.clone(),
            Err(poisoned) => poisoned.into_inner().audio_device.clone(),
        }
    }

    pub fn set_audio_device(&self, dev: Option<String>) {
        match self.0.write() {
            Ok(mut g) => g.audio_device = dev,
            Err(poisoned) => poisoned.into_inner().audio_device = dev,
        }
    }
}

impl Default for AppStateHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_state_get_set() {
        let h = AppStateHandle::new();
        assert_eq!(h.get_sample_rate(), 48000);
        h.set_sample_rate(96000);
        assert_eq!(h.get_sample_rate(), 96000);

        assert_eq!(h.get_audio_device(), None);
        h.set_audio_device(Some("hw:0".into()));
        assert_eq!(h.get_audio_device(), Some("hw:0".into()));
    }
}
