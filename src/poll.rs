use std::{io, time::Duration};
use crate::{sys, Events};

pub struct Poll {
    registry: Registry,
}

pub struct Registry {
    selector: sys::Selector,
}

impl Poll {
    pub fn register(&self) -> &Registry {
        &self.registry
    }
    pub fn new() -> io::Result<Poll> {
        sys::Selector::new().map(|selector| Poll {
            registry: Registry { selector},
        })
    }
    pub fn poll(& mut self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()>{
        self.registry.selector.select(events, timeout)
    }
}