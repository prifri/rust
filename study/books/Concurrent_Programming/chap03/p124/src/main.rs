/*
 * prifri, 2022.11.30:
 * - Condvar
 * 조건변수용 타입
 */
use std::sync::{Arc, Mutex, Condvar};
use std::{thread, time};

fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
/*
 * prifri, 2022.11.30:
 * - 튜플의 구성채를 나누는 방법인듯 싶다.
 */
    let &(ref lock, ref cvar) = &*p;

/*
 * prifri, 2022.11.30:
 * - wait_while함수를 통해서도 제작 가능하다고 한다.
 * - wait_timeout
 */
    println!("{} in", id);
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("child {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;
    let mut started = lock.lock().unwrap();

    *started = true;
    cvar.notify_all();
    println!("parent");
}

/*
 * prifri, 2022.11.30:
 * - 1개의 lock을 3개의 thread가 공유해서 c0, c1은 boardcast가 올때까지 잠들고 p가 깨우는 예제.
 */
fn main() {
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    let c0 = thread::spawn(move || { child(0, pair0) });
    let c1 = thread::spawn(move || { child(1, pair1) });
    
    let ms = time::Duration::from_millis(1000);
    thread::sleep(ms);
    let p = thread::spawn(move || { parent(pair2) });

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}
