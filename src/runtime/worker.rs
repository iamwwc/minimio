use std::{sync::Arc, vec};

use super::task::{self, Task};


type Local<T> = Vec<Task<T>>;
pub (super) struct Worker {
    local_queue: Local<task::Notified>
}
pub fn create_workers(size: usize) -> Vec<Arc<Worker>>{
    let workers = vec![];
    let one_queue = vec![];
    let a_one_queue = Arc::new(one_queue);
    for i in 0 .. size {
        workers.push(Arc::new(Worker {
            local_queue: a_one_queue.clone()
        }))
    }
    workers
}
impl Worker {
    pub fn run(&self) {
        while true {
            let task = self.local_queue.pop();
            
        }
    }

    pub fn next_task(self) {

    }
}