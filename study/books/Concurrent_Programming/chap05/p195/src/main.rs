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
 * - 명시적인 Future 타입 기술.
 *   await: Future 타입의 값이 결정될 때까지 처리를 정지
 *   async: Future 타입을 포함한 처리를 기술하기 위해 사용.
 *
 * - async { code } 라고 기술하는 경우 Future 트레이트를 구현한 타입이
 *   컴파일러에 의해 새롭게 정의되어 compile가 async { code } 부분에는
 *   해당 타입의 new 함수에 해당하는 호출이 이루어진다.
 *
 * - h.await는 다음과 같은 생략타입이다.
 *   match h.poll(cx) {
 *      Poll::Pending => return Poll:Pending,
 *      Poll::result(x) => x,
 *   }
 *
 * - 몬말인지 모르겠다 그냥 site 참고가 훨씬 나을듯
 *   https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
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
