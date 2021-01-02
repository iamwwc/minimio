use std::{sync::Arc, vec};

use super::task::Task;


type Local<T> = Vec<Task<T>>;
pub (super) struct Worker {
    local_queue: Arc<Local>
}
pub fn create_worker() -> Vec<Worker>{
    let workers = vec![];
    let one_queue = vec![];
    let counts = num_cpus::get();
    let a_one_queue = Arc::new(one_queue);
    for i in 0 .. counts {
        workers.push(Worker {
            local_queue: a_one_queue.clone()
        })
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