use std::io;

use crate::sys;

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
}