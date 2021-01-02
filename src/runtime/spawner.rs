use std::future::Future;

use super::threadpoll;
pub (crate) enum Spawner {
    ThreadPoll(threadpoll::Spawner),
}

impl Spawner {
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static, {

    } 
}