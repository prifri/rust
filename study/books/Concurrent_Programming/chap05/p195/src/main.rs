use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

struct Task {
    future: Mutex<BoxFuture<'static, ()>>,
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

struct Executor {
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>
}

impl Executor {
    fn new() -> Self {
        let (sender, receiver) = sync_channel(1024);
        Executor {
            sender,
            receiver
        }
    }

    fn get_spawner(&self) -> Spawner {
        Spawner {
            sender: self.sender.clone()
        }
    }

    fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            let mut future = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            if let Poll::Ready(_) = future.as_mut().poll(&mut ctx) {
                break;
            }
        }
    }
}

struct Spawner {
    sender: SyncSender<Arc<Task>>
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });

        self.sender.send(task).unwrap();
    }
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

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::Hello => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            StateHello::END => {
                Poll::Ready(())
            }
        }
    }
}

fn test1() {
    let executor = Executor::new();
/*
 * prifri, 2022.12.08:
 * - ???????????? Future ?????? ??????.
 *   await: Future ????????? ?????? ????????? ????????? ????????? ??????
 *   async: Future ????????? ????????? ????????? ???????????? ?????? ??????.
 *
 * - async { code } ?????? ???????????? ?????? Future ??????????????? ????????? ?????????
 *   ??????????????? ?????? ????????? ???????????? compile??? async { code } ????????????
 *   ?????? ????????? new ????????? ???????????? ????????? ???????????????.
 *
 * - h.await??? ????????? ?????? ??????????????????.
 *   match h.poll(cx) {
 *      Poll::Pending => return Poll:Pending,
 *      Poll::result(x) => x,
 *   }
 */
    executor.get_spawner().spawn(async {
        let h = Hello::new();
        h.await;
    });
    executor.run();
}

async fn hello() {
    let h = Hello::new();
    h.await;
}

fn test2() {
    let executor = Executor::new();
    executor.get_spawner().spawn( hello() );
    executor.run();
}

fn main() {
    test1();
    test2();
}
