use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use nix::{
    errno::Errno,
    sys::{
        epoll::{
            epoll_create1, epoll_ctl, epoll_wait,
            EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
        },
        eventfd::{eventfd, EfdFlags},
    },
    unistd::write,
};
use std:: {
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

fn write_eventfd(fd: RawFd, n: usize) {
    let ptr = &n as *const usize as *mut u8;
    let val = unsafe {
        std::slice::from_raw_parts_mut(
            ptr, std::mem::size_of_val(&n))
    };
    write(fd, &val).unwrap();
}

enum IOOps {
    ADD(EpollFlags, RawFd, Waker),
    REMOVE(RawFd),
}

struct IOSelector {
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
        let epoll_one = EpollFlags::EPOLLONESHOT;

        let mut ev = EpollEvent::new(flag | epoll_one, fd as u64);

        IF 
    }
}

fn main() {
    println!("Hello, world!");
}
