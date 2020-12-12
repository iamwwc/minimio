use std::{io, sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}, mpsc}};
use std::os::unix::io::{ AsRawFd, RawFd};
use crate::Token;
/// Selector
pub struct Selector {
    id: usize,
    ep: RawFd,
}
static NEXT_POLL_ID: AtomicUsize = AtomicUsize::new(1);
impl Selector {
    pub fn new () -> io::Result<Selector> {
        let flag = libc::EPOLL_CLOEXEC;
        syscall!(epoll_create1(flag)).map(|ep| Selector{
            ep: ep,
            id: NEXT_POLL_ID.fetch_add(1, Ordering::Relaxed),
        })
    }
    pub fn register(&self, fd: RawFd, token: Token) -> io::Result<()> {
        Ok(())
    }

    // 将 FD 从 selector 上解除注册
    pub fn unregister() -> io::Result<()> {
        Ok(())
    }
}