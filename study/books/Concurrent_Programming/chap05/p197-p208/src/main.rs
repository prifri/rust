use futures::{
    future::{BoxFuture, FutureExt},
    // task::{waker_ref, ArcWake}, io::ReadLine,
    task::{waker_ref, ArcWake},
};
use nix::{
    errno::Errno,
    sys::{
        epoll::{
            epoll_create1, epoll_ctl, epoll_wait,
            EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
        },
       /*
        * prifri, 2022.12.13:
        * - eventfd
        *   linux 고유의 이벤트 알림용 인터페이스.
        */
        eventfd::{eventfd, EfdFlags}, socket::SockAddr,
    },
    unistd::write,
};
use std:: {
/*
 * prifri, 2022.12.13:
 * - VecDeque
 *   vector list.
 */
    collections::{HashMap, VecDeque},
    future::Future,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    os::unix::io::{AsRawFd, RawFd},
    pin::Pin,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
};

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

macro_rules! dpr {
    () => {{
        println!("{} {}", function!(), line!());
    }}
}

/*
 * prifri, 2022.12.13:
 * - size_of_val(x)
 *   sizeof(*x)
 *
 * - from_raw_parts_mut(ptr[T], size)
 *   ptr을 size만큼의 &[T] format으로 전환한 값을 return.
 *   type의 변환일 뿐(컴파일러에게 알리는 용도)인거 같다. 내부적으로 변하거나
 *   값이 복사되는건 아닌듯.
 */
fn write_eventfd(fd: RawFd, n: usize) {
    let ptr = &n as *const usize as *mut u8;
    let val = unsafe {
        std::slice::from_raw_parts(
            ptr, std::mem::size_of_val(&n))
    };
/*
 * prifri, 2022.12.13:
 * - val 자체에 length 정보가 있으므로 c처럼 3번째인자에 length를 굳이
 *   안주는듯.
 */
    write(fd, &val).unwrap();
}

enum IOOps {
    ADD(EpollFlags, RawFd, Waker),
    REMOVE(RawFd),
}

struct IOSelector {
/*
 * prifri, 2022.12.13:
 * - hashmap을 waker로 추상화 시켜서 설명한다. fd(RawFd)를 key로 Waker를
 *   등록한다. 사실 그냥 fd가 한번이라도 들어왔는지 넣어놓는 자료구조.
 */
    wakers: Mutex<HashMap<RawFd, Waker>>,
    queue: Mutex<VecDeque<IOOps>>,
    epfd: RawFd,
    event: RawFd,
}

impl IOSelector {
    fn new() -> Arc<Self> {
        let s = IOSelector {
            wakers: Mutex::new(HashMap::new()),
            queue: Mutex::new(VecDeque::new()),
            epfd: epoll_create1(EpollCreateFlags::empty()).unwrap(),
            event: eventfd(0, EfdFlags::empty()).unwrap()
        };
        let result = Arc::new(s);
        let s = result.clone();

/*
 * prifri, 2022.12.13:
 * - epoll용 thread생성. 및 select() 동작
 */
        std::thread::spawn(move || s.select());
        result
    }

    fn add_event(
        &self,
        flag: EpollFlags,
        fd: RawFd,
        waker: Waker,
        wakers: &mut HashMap<RawFd, Waker>,
        ) {
        let epoll_add = EpollOp::EpollCtlAdd;
        let epoll_mod = EpollOp::EpollCtlMod;
/*
 * prifri, 2022.12.13:
 * - 이벤트한번 발생하면 재지정까지 다시 안발생.
 */
        let epoll_one = EpollFlags::EPOLLONESHOT;

        let mut ev = EpollEvent::new(flag | epoll_one, fd as u64);

/*
 * prifri, 2022.12.13:
 * - fd를 epfd에 추가.
 */
        if let Err(err) = epoll_ctl(self.epfd, epoll_add, fd,
                                    &mut ev) {
            match err {
/*
 * prifri, 2022.12.13:
 * - 이미 추가되있다면 재설정.
 *   책에서는 EPOLLONESHOT을 이해하게 하기위해 일부러 이렇게 했다고 한다.
 *   등록이 되있었다는걸 임의의 자료구조에 set해놓고 거기서 true가 떨어지면
 *   이걸 호출하고, 그게 아니면 epoll_ctl로 추가를 해야된다는 말이다.
 *   현재 함수 호출횟수가 적다면 이방법도 굳이 나쁘지 않은거같다.
 */
                nix::Error::Sys(Errno::EEXIST) => {
                    epoll_ctl(self.epfd, epoll_mod, fd,
                              &mut ev).unwrap();
                }
                _ => {
                    panic!("epoll_ctl: {}", err);
                }
            }
        }

        assert!(!wakers.contains_key(&fd));
        wakers.insert(fd, waker);
    }

    fn rm_event(&self, fd: RawFd, wakers: &mut HashMap<RawFd, Waker>) {
        let epoll_del = EpollOp::EpollCtlDel;
        let mut ev = EpollEvent::new(EpollFlags::empty(),
        fd as u64);
        epoll_ctl(self.epfd, epoll_del, fd, &mut ev).ok();
        wakers.remove(&fd); 
    }

    fn select(&self) {
        let epoll_in = EpollFlags::EPOLLIN;
        let epoll_add = EpollOp::EpollCtlAdd;

        let mut ev = EpollEvent::new(epoll_in,
                                     self.event as u64);
        epoll_ctl(self.epfd, epoll_add, self.event, &mut ev).unwrap();

        let mut events = vec![EpollEvent::empty(); 1024];
        while let Ok(nfds) = epoll_wait(self.epfd,
                                        &mut events, -1) {
            let mut t = self.wakers.lock().unwrap();
            for n in 0..nfds {
/*
 * prifri, 2022.12.13:
 * - 자기자신이라는것은 event가 발생(register, unregister) 했다는것.
 *   이벤트 직전에 등록할려는 fd가 queue에 넣어왔을것이다.
 *   queue가 다 빌때까지 fd를 event에 add한다.
 * - register, unregister요청 -> event로 trriger -> queue에서 등록된
 *   event확인후 처리
 */
                if events[n].data() == self.event as u64 {
                    let mut q = self.queue.lock().unwrap();
                    while let Some(op) = q.pop_front() {
                        match op {
                            IOOps::ADD(flag, fd, waker) =>
                                self.add_event(flag, fd, waker, &mut t),
                            IOOps::REMOVE(fd) =>
                                self.rm_event(fd, &mut t),
                        }
                    }
                } else {
/*
 * prifri, 2022.12.13:
 * - fd인 경우 실행 큐에 추가한다.
 */
                    let data = events[n].data() as i32;
                    let waker = t.remove(&data).unwrap();
                    waker.wake_by_ref();
                }
            }
        }
    }

    fn register(&self, flags: EpollFlags, fd: RawFd, waker: Waker) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(IOOps::ADD(flags, fd, waker));
        write_eventfd(self.event, 1);
    }

    fn unregister(&self, fd: RawFd) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(IOOps::REMOVE(fd));
        write_eventfd(self.event, 1);
    }
}

struct AsyncListener {
    listener: TcpListener,
    selector: Arc<IOSelector>,
}

/*
 * prifri, 2022.12.13:
 * - listen처리에 대한 비동기 처리.
 */
impl AsyncListener {
    fn listen(addr: &str, selector: Arc<IOSelector>) -> AsyncListener {
        let listener = TcpListener::bind(addr).unwrap();

        listener.set_nonblocking(true).unwrap();

        AsyncListener {
            listener,
            selector
        }
    }
    fn accept(&self) -> Accept {
        Accept { listener: self }
    }
}

impl Drop for AsyncListener {
    fn drop(&mut self) {
        self.selector.unregister(self.listener.as_raw_fd());
    }
}

struct Accept<'a> {
    listener: &'a AsyncListener,
}

impl<'a> Future for Accept<'a> {
    type Output = (AsyncReader,
                   BufWriter<TcpStream>,
                   SocketAddr);

    fn poll(self: Pin<&mut Self>,
            cx: &mut Context<'_>) -> Poll<Self::Output> {
/*
 * prifri, 2022.12.13:
 * - nonblocking 으로 동작. 
 */
        match self.listener.listener.accept() {
/*
 * prifri, 2022.12.13:
 * - listen성공시 read, write stream생성
 */
            Ok((stream, addr)) => {
                let stream0 = stream.try_clone().unwrap();
                Poll::Ready((
                    AsyncReader::new(stream0,
                    self.listener.selector.clone()), BufWriter::new(stream),
                    addr
                ))
            }
/*
 * prifri, 2022.12.13:
 * - 다 처리 됬으면(WouldBlock) listen fd을 다시 epoll로 돌려보내고 실행 중단.
 */
            Err(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    self.listener.selector.register(
                        EpollFlags::EPOLLIN,
                        self.listener.listener.as_raw_fd(),
                        cx.waker().clone(),
                        );
                    Poll::Pending
                } else {
                    panic!("accept: {}", err);
                }
            }
        }
    }
}

struct AsyncReader {
    fd: RawFd,
    reader: BufReader<TcpStream>,
    selector: Arc<IOSelector>,
}

/*
 * prifri, 2022.12.13:
 * - read처리에 대한 비동기 처리. listen과 비슷하게 동작한다
 */
impl AsyncReader {
    fn new(stream: TcpStream,
           selector: Arc<IOSelector>) -> AsyncReader {
        stream.set_nonblocking(true).unwrap();
        AsyncReader {
            fd: stream.as_raw_fd(),
            reader: BufReader::new(stream),
            selector: selector,
        }
    }

    fn read_line(&mut self) -> ReadLine {
        ReadLine { reader: self }
    }
}

impl Drop for AsyncReader {
    fn drop(&mut self) {
        self.selector.unregister(self.fd);
    }
}

struct ReadLine<'a> {
    reader: &'a mut AsyncReader,
}

impl<'a> Future for ReadLine<'a> {
    type Output = Option<String>;

    fn poll(mut self: Pin<&mut Self>,
            cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut line = String::new();
        match self.reader.reader.read_line(&mut line) {
/*
 * prifri, 2022.12.13:
 * - close.
 */
            Ok(0) => Poll::Ready(None),
            Ok(_) => Poll::Ready(Some(line)),
/*
 * prifri, 2022.12.13:
 * - 다 읽은 경우. pending. error면 close
 */
            Err(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    self.reader.selector.register(
                        EpollFlags::EPOLLIN,
                        self.reader.fd,
                        cx.waker().clone(),
                        );
                    Poll::Pending
                } else {
                    Poll::Ready(None)
                }
            }

        }
    }
}

struct Task {
    future: Mutex<BoxFuture<'static, ()>>,
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let self0 = arc_self.clone();
        dpr!();
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
            let _ = future.as_mut().poll(&mut ctx);
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

fn main() {
    dpr!();
    let executor = Executor::new();
    dpr!();
    let selector = IOSelector::new();
    dpr!();
    let spawner = executor.get_spawner();
    dpr!();

/*
 * prifri, 2022.12.13:
 * - future 객체 생성
 */
    let server = async move {
    dpr!();
        let listener = AsyncListener::listen("127.0.0.1:10001",
                                             selector.clone());
    dpr!();
        loop {
    dpr!();
            let (mut reader, mut writer, addr) =
                listener.accept().await;
            println!("accept: {}", addr);

            spawner.spawn(async move {
                while let Some(buf) = reader.read_line().await {
                    print!("read: {}. {}", addr, buf);
                    writer.write(buf.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
            });
            println!("close: {}", addr);
        }
    };

    dpr!();
/*
 * prifri, 2022.12.13:
 * - task 생성
 */
    executor.get_spawner().spawn(server);
    dpr!();
/*
 * prifri, 2022.12.13:
 * - task 실행
 */
    executor.run();
    dpr!();
}
