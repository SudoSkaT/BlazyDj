use lazydj_events::{Event, EventBus};

pub enum Command {
    Play(u8),
    Pause(u8),
    Stop(u8),
}

pub struct CommandDispatcher {
    bus: EventBus,
}

impl CommandDispatcher {
    pub fn new(bus: EventBus) -> Self {
        Self { bus }
    }

    pub fn dispatch(&self, cmd: Command) {
        match cmd {
            Command::Play(deck) => self.bus.publish(Event::DeckPlay(deck)),
            Command::Pause(deck) => self.bus.publish(Event::DeckPause(deck)),
            Command::Stop(deck) => self.bus.publish(Event::DeckPause(deck)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn dispatch_play_publishes_event() {
        let bus = EventBus::new();
        let r = bus.subscribe();
        let d = CommandDispatcher::new(bus.clone());
        d.dispatch(Command::Play(1));
        let ev = r.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(ev, Event::DeckPlay(1));
    }
}
