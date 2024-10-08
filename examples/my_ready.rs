use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[tokio::main]
async fn main() {
    // let ret = poll_fut(&mut Context::from_waker(futures::task::noop_waker_ref()));
    // println!("{:?}", ret);
    let fut = MyFut::new(42);
    println!("{:?}", fut.await);
}

#[allow(unused)]
fn poll_fut(cx: &mut Context<'_>) -> Poll<usize> {
    // let mut fut = future::ready(42);

    let mut fut = MyFut::new(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}

struct MyFut {
    polled: bool,
    v: usize,
}

impl MyFut {
    fn new(v: usize) -> Self {
        Self { polled: false, v }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(v) => std::task::Poll::Ready(v),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
