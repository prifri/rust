use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

enum StateHello {
    Hello,
    WORLD,
    END,
}

struct Hello {
    state: StateHello,
}

impl Hello {
    fn new() -> Self {
        Hello {
            state: StateHello::Hello,
        }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::Hello => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
                Poll::Pending
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                Poll::Pending
            }
            StateHello::END => {
                Poll::Ready(())
            }
        }
    }
}

struct Task {
    hello: Mutex<BoxFuture<'static, ()>>,
}

impl Task {
    fn new() -> Self {
        let hello = Hello::new();
        Task {
            hello: Mutex::new(hello.boxed()),
        }
    }
}

/*
 * prifri, 2022.12.08:
 * - https://www.snoyman.com/blog/2019/12/rust-crash-course-08-down-dirty-future/
 */
fn main() {
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker);
    let mut hello = task.hello.lock().unwrap();

    let result = hello.as_mut().poll(&mut ctx);
    match result {
        Poll::Pending => {
            println!("{}", line!());
        }
        Poll::Ready(_) => {
            println!("{}", line!());
        }
    }
    let result = hello.as_mut().poll(&mut ctx);
    match result {
        Poll::Pending => {
            println!("{}", line!());
        }
        Poll::Ready(_) => {
            println!("{}", line!());
        }
    }
    let result = hello.as_mut().poll(&mut ctx);
    match result {
        Poll::Pending => {
            println!("{}", line!());
        }
        Poll::Ready(_) => {
            println!("{}", line!());
        }
    }
    let result = hello.as_mut().poll(&mut ctx);
    match result {
        Poll::Pending => {
            println!("{}", line!());
        }
        Poll::Ready(_) => {
            println!("{}", line!());
        }
    }
}
