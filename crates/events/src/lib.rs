use flume::{Receiver, Sender, unbounded};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    TrackLoaded(String),
    DeckPlay(u8),
    DeckPause(u8),
}

#[derive(Clone)]
pub struct EventBus {
    subscribers: Arc<Mutex<Vec<Sender<Event>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe(&self) -> Receiver<Event> {
        let (s, r) = unbounded();
        // Avoid unwrap/expect in production code; handle poisoning by taking inner
        match self.subscribers.lock() {
            Ok(mut guard) => guard.push(s),
            Err(poisoned) => poisoned.into_inner().push(s),
        }
        r
    }

    pub fn publish(&self, ev: Event) {
        // Clone list of subscribers while holding lock, then release lock
        let subscribers = match self.subscribers.lock() {
            Ok(guard) => guard.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        };

        for s in subscribers {
            // best-effort delivery; ignore send errors
            let _ = s.send(ev.clone());
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_receive() {
        let bus = EventBus::new();
        let r1 = bus.subscribe();
        let r2 = bus.subscribe();

        bus.publish(Event::TrackLoaded("track1".into()));

        assert_eq!(r1.recv().unwrap(), Event::TrackLoaded("track1".into()));
        assert_eq!(r2.recv().unwrap(), Event::TrackLoaded("track1".into()));
    }
}
