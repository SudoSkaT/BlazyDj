//! Cue system crate: hot cues, memory cues, loops (Phase 8 foundation).
//!
//! This crate is intentionally lightweight and sync-friendly; persistence and
//! DB wiring will be added later in a background worker.

use std::collections::HashMap;

/// Identifier for a track (matches DB row id type used elsewhere)
pub type TrackId = i64;

/// Kind of cue
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CueKind {
    Hot,
    Memory,
}

/// A cue point placed in a track
#[derive(Debug, Clone, PartialEq)]
pub struct CuePoint {
    pub id: u32,
    pub position_seconds: f32,
    pub label: Option<String>,
    pub kind: CueKind,
}

/// Loop in/out markers
#[derive(Debug, Clone, PartialEq)]
pub struct LoopPoint {
    pub in_seconds: f32,
    pub out_seconds: f32,
}

/// Per-track cue set
#[derive(Debug, Clone, PartialEq)]
pub struct CueSet {
    pub track_id: Option<TrackId>,
    hot_cues: Vec<CuePoint>,
    memory_cues: HashMap<usize, CuePoint>,
    pub loop_point: Option<LoopPoint>,
    next_id: u32,
}

impl Default for CueSet {
    fn default() -> Self {
        Self {
            track_id: None,
            hot_cues: Vec::new(),
            memory_cues: HashMap::new(),
            loop_point: None,
            next_id: 1,
        }
    }
}

impl CueSet {
    /// Create a new CueSet, optionally associated with a track id
    pub fn new(track_id: Option<TrackId>) -> Self {
        Self {
            track_id,
            ..Default::default()
        }
    }

    /// Add a hot cue; returns cue id
    pub fn add_hot_cue(&mut self, position_seconds: f32, label: Option<String>) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let cue = CuePoint {
            id,
            position_seconds,
            label,
            kind: CueKind::Hot,
        };
        self.hot_cues.push(cue);
        id
    }

    /// Remove a hot cue by id
    pub fn remove_hot_cue(&mut self, id: u32) -> Option<CuePoint> {
        if let Some(pos) = self.hot_cues.iter().position(|c| c.id == id) {
            Some(self.hot_cues.remove(pos))
        } else {
            None
        }
    }

    /// List hot cues
    pub fn hot_cues(&self) -> &[CuePoint] {
        &self.hot_cues
    }

    /// Set a memory cue at a slot index (overwrites existing)
    pub fn set_memory_cue(&mut self, slot: usize, position_seconds: f32, label: Option<String>) {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let cue = CuePoint {
            id,
            position_seconds,
            label,
            kind: CueKind::Memory,
        };
        self.memory_cues.insert(slot, cue);
    }

    /// Get memory cue by slot
    pub fn get_memory_cue(&self, slot: usize) -> Option<&CuePoint> {
        self.memory_cues.get(&slot)
    }

    /// Remove memory cue by slot
    pub fn remove_memory_cue(&mut self, slot: usize) -> Option<CuePoint> {
        self.memory_cues.remove(&slot)
    }

    /// Set loop in/out markers
    pub fn set_loop(&mut self, in_seconds: f32, out_seconds: f32) {
        self.loop_point = Some(LoopPoint {
            in_seconds,
            out_seconds,
        });
    }

    /// Clear loop markers
    pub fn clear_loop(&mut self) {
        self.loop_point = None;
    }

    /// Get loop markers
    pub fn loop_point(&self) -> Option<&LoopPoint> {
        self.loop_point.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_remove_hot_cue() {
        let mut cs = CueSet::new(Some(42));
        let id1 = cs.add_hot_cue(12.34, Some("start".to_string()));
        let id2 = cs.add_hot_cue(45.6, None);
        assert_eq!(cs.hot_cues().len(), 2);
        assert!(cs.hot_cues().iter().any(|c| c.id == id1));
        assert!(cs.hot_cues().iter().any(|c| c.id == id2));
        let removed = cs.remove_hot_cue(id1).expect("should remove");
        assert_eq!(removed.position_seconds, 12.34);
        assert_eq!(cs.hot_cues().len(), 1);
    }

    #[test]
    fn memory_cues_set_get_remove() {
        let mut cs = CueSet::new(None);
        cs.set_memory_cue(0, 5.0, Some("mem0".to_string()));
        cs.set_memory_cue(2, 10.0, None);
        let m0 = cs.get_memory_cue(0).expect("exists");
        assert_eq!(m0.position_seconds, 5.0);
        let removed = cs.remove_memory_cue(0).expect("removed");
        assert_eq!(removed.position_seconds, 5.0);
        assert!(cs.get_memory_cue(0).is_none());
    }

    #[test]
    fn loop_set_clear() {
        let mut cs = CueSet::new(None);
        cs.set_loop(1.0, 3.5);
        let lp = cs.loop_point().expect("loop set");
        assert_eq!(lp.in_seconds, 1.0);
        assert_eq!(lp.out_seconds, 3.5);
        cs.clear_loop();
        assert!(cs.loop_point().is_none());
    }
}
