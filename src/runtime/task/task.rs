use std::{cell::UnsafeCell, future::Future, marker::PhantomData, pin::Pin, ptr::NonNull, sync::atomic::AtomicUsize, task::{Context, Poll, Waker}};

pub struct Task<T> {
    pub ptr: NonNull<()>,
    _marker: PhantomData<T>
}
pub struct TaskVTable {
    // table中的 schedule 作用是二次调用 RawTask上的schdule
    // 放到 global queue 上
    pub schedule: unsafe fn (*const()),
    pub run: unsafe fn (),
}

pub struct Header {
    pub state: AtomicUsize,
    // .await认为是同步写法，所以.await之后的Task waker
    // 我们完成后就通过后面的waker，可以重新调度了
    pub awaiter: UnsafeCell<Option<Waker>>,
    pub table: &'static TaskVTable
}
pub struct RawTask<T, S, F> {
    header: *const Header,
    schedule: *const S,
    future: *mut F,
    output: *mut T
}

impl<T> Task<T> {
    pub fn poll_task(&mut self, ctx: &mut Context) -> Poll<Option<T>> {
        let ptr = self.ptr.as_ptr();
        // *const T和 * mut T 区别是
        // *const T 指针本身不可变，由于Cpp中有 const * 的写法声明不可变指针
        // 所以rust涉及到 FFI 时 *const T 合适
        // https://internals.rust-lang.org/t/what-is-the-real-difference-between-const-t-and-mut-t-raw-pointers/6127
        let header = ptr as *const Header;
    }
}

impl<T> Future for Task<T> {
    type Output = T;
    // Task实现Future类似实现 JoinHandle
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        match self.poll_task(&mut cx) {
            Poll::Ready(t) => Poll::Ready(t.expect("task has failed")),
            Poll::Pending => Poll::Pending,
        }
    }
}