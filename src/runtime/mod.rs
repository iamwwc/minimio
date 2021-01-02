mod context;
mod task;
mod threadpoll;
mod handle;
mod worker;
mod spawner;
mod schedule;
mod blocking;
use std::vec;

pub (crate) use threadpoll::*;

use handle::Handle;

pub struct Runtime {
    handle: Handle
}
pub fn build_threaded_runtime() -> Runtime {
    let workers = vec![];
    let counts = num_cpus::get();
    for _ in 0 .. counts {
        
    }
}