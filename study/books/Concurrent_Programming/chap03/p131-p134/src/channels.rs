use crate::semaphore::Semaphore;
use std::collections::LinkedList;
use std::sync::{Arc, Condvar, Mutex};

/*
 * prifri, 2022.12.02:
 * - lock을 두개 사용한다.
 *   1. 전송중인 개수를 알기위한 sema 
 *   2. wakeup을 기다리고 데이터를 넣기위한 buf 
 * - sema로 첫 대기줄을 걸고, sema를 얻은 thread만 buf lock을 얻어 data를 넣고
 *   buf wakeup을 하는 방식
 */
#[derive(Clone)]
pub struct Sender<T> {
    sem: Arc<Semaphore>,
    buf: Arc<Mutex<LinkedList<T>>>,
    cond: Arc<Condvar>,
}

impl<T: Send> Sender<T> {
/*
 * prifri, 2022.12.02:
 * - sema wait를 하고 list에 넣고 buf wakeup시킨다.
 *   이미 max개이상이 send wait중이라면 wait가 걸릴것이고, 그게아니면 list에
 *   값을 넣을것이다.
 */
    pub fn send(&self, data: T) {
        self.sem.wait();
        let mut buf = self.buf.lock().unwrap();
        buf.push_back(data);
        self.cond.notify_one();
    }
}

pub struct Receiver<T> {
    sem: Arc<Semaphore>,
    buf: Arc<Mutex<LinkedList<T>>>,
    cond: Arc<Condvar>,
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> T {
/*
 * prifri, 2022.12.02:
 * - buf lock wait. 누가 buf wakeup을 해주면 list에서 꺼내서 sema post를 한후
 *   return 한다.
 */
        let mut buf = self.buf.lock().unwrap();
        loop {
            if let Some(data) = buf.pop_front() {
                self.sem.post();
                return data;
            }

            buf = self.cond.wait(buf).unwrap();
        }
    }
}

pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);
    let sem = Arc::new(Semaphore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cond = Arc::new(Condvar :: new());
    let tx = Sender {
        sem: sem.clone(),
        buf: buf.clone(),
        cond: cond.clone(),
    };
    let rx = Receiver { sem, buf, cond };
    (tx, rx)
}
