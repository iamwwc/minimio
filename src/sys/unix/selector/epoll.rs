use std::{cmp, io, ptr, sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}, mpsc}, time::Duration};
use std::os::unix::io::{ AsRawFd, RawFd};
use libc::{EPOLLET, EPOLLIN, EPOLLOUT, EPOLLRDHUP, epoll_ctl, syscall};

use crate::{Interest, Token};
/// Selector
pub struct Selector {
    id: usize,
    ep: RawFd,
}
pub type Event = libc::epoll_event;
pub type Events = Vec<Event>;
static NEXT_POLL_ID: AtomicUsize = AtomicUsize::new(1);
impl Selector {
    pub fn new () -> io::Result<Selector> {
        let flag = libc::EPOLL_CLOEXEC;
        syscall!(epoll_create1(flag)).map(|ep| Selector{
            ep: ep,
            id: NEXT_POLL_ID.fetch_add(1, Ordering::Relaxed),
        })
    }

    pub fn select(&self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
        const MAX_SAFE_TIMEOUT: u128 = 1789569;
        let timeout = timeout.map(|to| cmp::min(MAX_SAFE_TIMEOUT, to.as_millis()) as libc::c_int).unwrap_or(-1);

        events.clear();
        syscall!(epoll_wait(
            self.ep,
            events.as_mut_ptr(),
            events.capacity() as i32,
            timeout
        )).map(|n| {
            unsafe {
                events.set_len(n as usize);
            }
        })
    }
    pub fn register(&self, fd: RawFd, token: Token, interest: Interest) -> io::Result<()> {
        let mut event = libc::epoll_event{
            events: interest_to_events(interest),
            u64: usize::from(token) as u64,
        };
        syscall!(epoll_ctl(self.ep, libc::EPOLL_CTL_ADD, fd, &mut event)).map(|_|())
    }

    pub fn reregister(&self, fd: RawFd, token: Token, interest: Interest) -> io::Result<()> {
        let mut event = libc::epoll_event {
            events: interest_to_events(interest),
            u64: usize::from(token) as u64,
        };
        syscall!(epoll_ctl(self.ep, libc::EPOLL_CTL_MOD, fd, &mut event)).map(|_|())
    }

    // 将 FD 从 selector 上解除注册
    pub fn unregister(&self, fd: RawFd) -> io::Result<()> {
        syscall!(epoll_ctl(self.ep, libc::EPOLL_CTL_DEL, fd, ptr::null_mut())).map(|_|())
    }
}

fn interest_to_events (interest: Interest) -> u32 {
    let mut kind = EPOLLET;
    if interest.is_readable() {
        kind = kind | EPOLLIN | EPOLLRDHUP;
    }
    if interest.is_writable() {
        kind = kind | EPOLLOUT;
    }
    kind as u32
}