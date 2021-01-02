use crate::Spawner;

// Runtime 句柄

#[derive(Debug, Clone)]
pub struct Handle {
    io_handler: IoDriver,
    spawer: Spawner,
}