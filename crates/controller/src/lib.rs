//! Controller crate: device detection and mapping engine (Phase 10 foundation).
//!
//! Provides mock device detection (MIDI/HID), device profiles, and a simple
//! mapping engine. Real platform bindings (midir/hidapi) should be added behind
//! feature flags and implemented in background workers to avoid audio-thread work.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    Midi,
    Hid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
}

/// Mock detection: returns example MIDI and HID devices. Real detection
/// should be feature-gated and platform-specific.
pub fn detect_devices() -> Vec<DeviceInfo> {
    vec![
        DeviceInfo {
            id: String::from("mock-midi-1"),
            name: String::from("Mock MIDI Device"),
            device_type: DeviceType::Midi,
        },
        DeviceInfo {
            id: String::from("mock-hid-1"),
            name: String::from("Mock HID Device"),
            device_type: DeviceType::Hid,
        },
    ]
}

/// Device profile describing vendor/product and default control mappings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceProfile {
    pub vendor: String,
    pub product: String,
    pub mappings: HashMap<String, String>,
}

impl DeviceProfile {
    pub fn new(vendor: &str, product: &str) -> Self {
        Self {
            vendor: vendor.to_string(),
            product: product.to_string(),
            mappings: HashMap::new(),
        }
    }

    pub fn map(&mut self, control: &str, action: &str) {
        self.mappings
            .insert(control.to_string(), action.to_string());
    }

    pub fn mapping(&self, control: &str) -> Option<&String> {
        self.mappings.get(control)
    }
}

/// Lightweight mapping engine: maps (device_id, control) -> action name.
#[derive(Debug, Clone)]
pub struct MappingEngine {
    map: HashMap<(String, String), String>,
}

impl MappingEngine {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn map_control(&mut self, device_id: &str, control: &str, action: &str) {
        self.map.insert(
            (device_id.to_string(), control.to_string()),
            action.to_string(),
        );
    }

    pub fn remove_mapping(&mut self, device_id: &str, control: &str) -> Option<String> {
        self.map
            .remove(&(device_id.to_string(), control.to_string()))
    }

    /// Handle an incoming control event. Returns the mapped action, if any.
    pub fn handle_control(&self, device_id: &str, control: &str, _value: u8) -> Option<String> {
        self.map
            .get(&(device_id.to_string(), control.to_string()))
            .cloned()
    }
}

impl Default for MappingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_devices_returns_mock() {
        let devs = detect_devices();
        assert!(
            devs.iter()
                .any(|d| matches!(d.device_type, DeviceType::Midi))
        );
        assert!(
            devs.iter()
                .any(|d| matches!(d.device_type, DeviceType::Hid))
        );
    }

    #[test]
    fn mapping_engine_maps() {
        let mut eng = MappingEngine::new();
        eng.map_control("mock-midi-1", "cc:14", "play_pause");
        let action = eng.handle_control("mock-midi-1", "cc:14", 127);
        assert_eq!(action.as_deref(), Some("play_pause"));
        let removed = eng.remove_mapping("mock-midi-1", "cc:14");
        assert_eq!(removed.as_deref(), Some("play_pause"));
    }

    #[test]
    fn device_profile_maps() {
        let mut prof = DeviceProfile::new("Acme", "MockDeck");
        prof.map("fader:1", "volume");
        assert_eq!(prof.mapping("fader:1").map(|s| s.as_str()), Some("volume"));
    }
}
