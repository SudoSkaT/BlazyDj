//! UI foundation crate. Lightweight layout/state objects for Phase 6.

/// Layout state for desktop UI scaffolding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutState {
    /// Show library panel
    pub show_library: bool,
    /// Show deck panel
    pub show_deck: bool,
    /// Show mixer panel
    pub show_mixer: bool,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            show_library: true,
            show_deck: true,
            show_mixer: true,
        }
    }
}

impl LayoutState {
    /// Toggle library visibility
    pub fn toggle_library(&mut self) {
        self.show_library = !self.show_library;
    }

    /// Toggle deck visibility
    pub fn toggle_deck(&mut self) {
        self.show_deck = !self.show_deck;
    }

    /// Toggle mixer visibility
    pub fn toggle_mixer(&mut self) {
        self.show_mixer = !self.show_mixer;
    }
}

#[cfg(test)]
mod tests {
    use super::LayoutState;

    #[test]
    fn default_layout_shows_panels() {
        let s = LayoutState::default();
        assert!(s.show_library);
        assert!(s.show_deck);
        assert!(s.show_mixer);
    }

    #[test]
    fn toggles_work() {
        let mut s = LayoutState::default();
        s.toggle_library();
        assert!(!s.show_library);
        s.toggle_deck();
        assert!(!s.show_deck);
        s.toggle_mixer();
        assert!(!s.show_mixer);
    }
}
