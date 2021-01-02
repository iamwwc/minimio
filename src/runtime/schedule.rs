use std::collections::VecDeque;

use super::task;

pub trait Schedule {
    fn schedule(&self, task: task::Notified<Self>);
}

pub struct BasicScheduler {
    run_queue: VecDeque<task::Notified<Self>>
}

impl Schedule for BasicScheduler {
    fn schedule(&self, task: task::Notified<Self>) {

    }
}