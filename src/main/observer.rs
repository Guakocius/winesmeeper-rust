use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Event {
    Turn,
    FieldOpened,
    FlagPlaced,
    GameWon,
    GameLost,
}

#[derive(Clone)]
pub enum EventData {
    Turn { x: usize, y: usize },
    FieldOpened { x: usize, y: usize, bomb: bool},
    FlagPlaced { x: usize, y: usize },
    GameWon,
    GameLost,
}

type ObserverId = usize;

pub struct Observer {
    next_id: ObserverId,
    listeners: HashMap<Event, Vec<(ObserverId, Box<dyn FnMut(&EventData)>)>>,
}

impl Observer {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            listeners: HashMap::new(),
        }
    }

    pub fn subscribe<F>(&mut self, event_type: Event, callback: F) -> ObserverId
    where 
        F: FnMut(&EventData) + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        
        self.listeners
            .entry(event_type)
            .or_default()
            .push((id, Box::new(callback)));
        id
    }

    pub fn unsubscribe(&mut self, event_type: &Event, id: ObserverId) {
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            listeners.retain(|(listener_id, _)| *listener_id != id);
        }
    }

    pub fn notify(&mut self, event_type: &Event, data: EventData) {
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            for (_, listener) in listeners {
                listener(&data);
            }
        }
    }
}
