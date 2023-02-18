use std::collections::HashMap;

pub struct EventDispatch {
    events: HashMap<String, Box<dyn FnMut() + Send>>,
}

impl EventDispatch {
    pub fn new() -> EventDispatch {
        EventDispatch {
            events: HashMap::new(),
        }
    }

    pub fn register_event(&mut self, name: &str, event: impl FnMut() + Send + 'static) {
        self.events.insert(name.to_string(), Box::new(event));
    }

    pub async fn call_event(&mut self, name: &str) {
        if let Some(event) = self.events.get_mut(name) {
            event();
        }
    }
}
