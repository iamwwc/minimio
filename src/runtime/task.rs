use std::{future::Future, marker::PhantomData, ptr::NonNull, sync::atomic::AtomicUsize};

use super::schedule::Schedule;

pub struct Notified<S: 'static>(Task<S>);
pub struct Task<T> {
    pub(crate) ptr: NonNull<()>,
    pub(crate) _marker: PhantomData<T>,
}

pub struct TaskVtable {
    pub (crate) poll: unsafe fn(NonNull<Header>),
    pub (crate) dealloc: unsafe fn(NonNull<Header>),
}
pub struct Header {
    pub (crate) state: AtomicUsize,
    pub (crate) vtable: &'static TaskVtable
}
pub struct RawTask {
    pub (crate) ptr: NonNull<Header>
}

pub struct Cell {
    header: Header,
    future: 
}
impl Task<T> {
    pub fn new<T, S>(task: T) -> Task 
    where
        T: Future
    {
        Box::into_raw(Task {

        })   

    }
}

struct JoinHandle<T>{
    raw: RawTask,
    _p
}
pub fn joinable<T, S>(f: T) -> JoinHandle<T::Output>
    where 
        T: Future + Send + 'static,
        S: Schedule
{

}