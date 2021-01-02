use std::{future::Future, pin::Pin, task::{Context, Poll}};

pub struct BlockingTask<T> {
    // 如果 func 可能为 null 就用 Option 包装下
    func: Option<T>
}

impl <T, R>Future for BlockingTask<T> 
    where 
        T: FnOnce() -> R + Send + 'static,
        R: Send + 'static
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<R> {
        let me = &mut *self;
        let func = me.func.take().expect("Blocking Task run twice!");
        Poll::Ready(func())
    }
}