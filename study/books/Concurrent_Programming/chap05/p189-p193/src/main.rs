use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

/*
 * prifri, 2022.12.08:
 * - callback function한개를 enum으로 구간을 나누어 관리한다. 이것을
 *   task라고 여기서 명칭한다.
 *   그 callback을 실행하는것을 executor라고 한다.
 *   executor는 recv -> callback 실행 -> recv..를 반복한다.
 *   executor의 recv가 작동하게 하기 위해 send를 하는게 waker라는 개념이며,
 *   이 예제에서는 callback이 자기자신을 wake하므로 callback(task)와
 *   waker를 동일시 한다. 즉
 *
 *   executor recv -> task가 executor에
 *   자기자신을 waker요청 -> callback(task)실행 -> tsk종료
 *   -> executor recv엔 직전에 waker요청이 있으므로 다시 실행.
 *
 *   별거없이 callback 함수를 반복하게 하는 일련의 동작일뿐인데,
 *   다만 이런 일련의 과정에 대해 통일된 객체와 규칙을 적용했다.
 *   그런데 이 규칙이란게 꽤 복잡하다.
 *
 *   이런 일련의 과정을 정형화 하기 위해서 여러 개념과 저런 future api들의
 *   인자가 필요하다고 설계하여 저런식으로 부르도록 강제한거같다.
 *
 *   executor : task실행. 여기서는 task.poll을 실행한다.
 *   context  : task를 관리하기 위한 객체. waker정보를 담아 놓았다.
 *   waker    : executor에 wake요청을 하기 위한것.
 *   task     : callback 시행을 위한것.
 *   task.poll : context를 인자로 받게하여 task실행후 무언가를 할수있게함.
 *               여기서는 waker를 context에 담앗다.
 *
 *   그냥 코루틴의 동작원리의 기본을 이렇게 구현된다고만 이해하고 넘어가도
 *   될거같다.
 */

//**********************************************************************8
// Task & Waker
//**********************************************************************8

/*
 * prifri, 2022.12.08:
 * - waker의 기능도 같이 있다.
 *   wake란게 별게 아니고 큐에 자기자신을 넣어놓는것뿐.
 */
struct Task {
    /*
     * prifri, 2022.12.08:
     * - 실행할 코루틴(future). 이 future의 실행을 완료할 때까지 executor가
     * 실행을 수행한다.
     */
    future: Mutex<BoxFuture<'static, ()>>,
    /*
     * prifri, 2022.12.08:
     * - executor로 task를 전달하고 스케줄링을 수행하기 위한 채널.
     */
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
/*
 * prifri, 2022.12.08:
 * - 자신의 Arc참조를 executor로 송신하고 스케쥴링한다..
 */
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

//**********************************************************************8
// Executor
//**********************************************************************8

/*
 * prifri, 2022.12.08:
 * - Executor
 *   recv -> tsk 실행 -> recv 반복.
 *   이때 task를 실행할때 
 */
struct Executor {
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>
}

impl Executor {
    fn new() -> Self {
        let (sender, receiver) = sync_channel(1024);
/*
 * prifri, 2022.12.08:
 * - 굳이 clone을 왜했을까? get_spawner에서만 clone을 하면 될거같다.
 */
        Executor {
            //sender: sender.clone(),
            sender,
            receiver
        }
    }
/*
 * prifri, 2022.12.08:
 * - 새로운 task를 생성하고 실행 큐에 넣기위한 객체를 반환.
 */

    fn get_spawner(&self) -> Spawner {
        Spawner {
            sender: self.sender.clone()
        }
    }

/*
 * prifri, 2022.12.08:
 * -채널에서 task를 수신해서 실행한다.
 */
    fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            let mut future = task.future.lock().unwrap();
/*
 * prifri, 2022.12.08:
 * - waker_ref(W)
 *   Task 정보를 가지는 Waker를 만든다.
 * - Waker
 *   task를 스케쥴링할때 이용된다.
 * - std::task::Context
 *   https://doc.rust-lang.org/stable/std/task/struct.Context.html 
 *   비동기 task의 context
 * - Context::from_waker
 *   &Waker로부터 Context를 생성한다.
 */
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
/*
 * prifri, 2022.12.08:
 * - main에서 executor가 동작하니 그냥 종료시켜보고 싶어서 이렇게 했다.
 *   코루틴만을 동작하기위해 띄워놓을거면 executor를 thread화해서 따로
 *   띄우는식으로 해야된다.
 */
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
/*
 * prifri, 2022.12.08:
 * - task를 생성해서 실행 큐에 추가한다. future를 받아 Box화해서 Task에
 *   감싸서 실행 큐에 넣는다.
 * - +'static + Send
 *   BoxFuture가
 *   pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a, Global>>; 
 *   로 정의되서 그렇다. 왜그런진 너무 어렵다. 일단 그정도로만 알고 넘어가겠다.
 */
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });

/*
 * prifri, 2022.12.08:
 * - spawn즉시 실행 큐에 넣어놓으므로 최초의 recv에 즉시 실행될것이다.
 */
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

/*
 * prifri, 2022.12.08:
 * - 실행이 다됫으면 context를 통해 자기자신을 다시 실행 queue에 넣고 종료한다.
 */
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::Hello => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
/*
 * prifri, 2022.12.08:
 * - Context::waker()
 *   Current tsk에 대한 Waker Ref를 반환한다.
 */
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
/*
 * prifri, 2022.12.08:
 * - 코루틴을 무한반복 시킬거면 state를 hello로 갱신하고
 *   wake 시킨다음에 pending시키면된다.
 */
                Poll::Ready(())
            }
        }
    }
}

fn main() {
    let executor = Executor::new();
    executor.get_spawner().spawn(Hello::new());
    executor.run();
}
