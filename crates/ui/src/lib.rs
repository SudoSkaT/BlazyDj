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

/// Library panel state: search query and selection
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LibraryViewState {
    pub query: String,
    pub selected_index: Option<usize>,
}

/// Deck view state for a single deck
#[derive(Debug, Clone, PartialEq)]
pub struct DeckViewState {
    pub track_id: Option<i64>,
    pub playing: bool,
    pub volume: f32,
    pub position_seconds: f32,
}

impl Default for DeckViewState {
    fn default() -> Self {
        Self {
            track_id: None,
            playing: false,
            volume: 1.0,
            position_seconds: 0.0,
        }
    }
}

/// Mixer view state
#[derive(Debug, Clone, PartialEq)]
pub struct MixerViewState {
    pub crossfader: f32, // -1.0 left, 0 center, 1.0 right
    pub gain_left: f32,
    pub gain_right: f32,
}

impl Default for MixerViewState {
    fn default() -> Self {
        Self {
            crossfader: 0.0,
            gain_left: 1.0,
            gain_right: 1.0,
        }
    }
}

/// Status bar state
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StatusBarState {
    pub message: Option<String>,
}

/// Theme enum for UI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

/// Aggregate UI state
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UIState {
    pub layout: LayoutState,
    pub library: LibraryViewState,
    pub deck_a: DeckViewState,
    pub deck_b: DeckViewState,
    pub mixer: MixerViewState,
    pub status: StatusBarState,
    pub theme: Theme,
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn ui_state_default_and_components() {
        let u = UIState::default();
        assert!(u.layout.show_library);
        assert_eq!(u.deck_a.volume, 1.0);
        assert_eq!(u.deck_b.volume, 1.0);
        assert_eq!(u.mixer.crossfader, 0.0);
        assert!(matches!(u.theme, Theme::Dark));
    }
}
