/*
 * prifri, 2022.11.30:
 * - Arc
 * thread safe한 참조 카운터 타입의 스마트 포인터
 */
use std::sync::{Arc, Mutex};
use std::thread;

fn some_func(lock: Arc<Mutex<u64>>) {
/*
 * prifri, 2022.11.30:
 * - loop안에서 lock을걸고, loop scope를 벗어날경우 자동으로 unlock된다.
 * - try_lock
 *   lock획득만 시도할수있는 함수.
 */
    loop {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{}", *val);
    }
}

/*
 * prifri, 2022.11.30:
 * - C에서는 하나의 lock을 가지고 여러군데에서 &lock 이렇게 해서 그냥 접근하지만
 *   rust에선 소유권때문에 lock.clone()으로 참조카운터를 늘려서 가져오는 식으로 하는듯하다.
 * - 또, C와 다르게, 특정 값에 대해서만 lock를 얻어올수있는 구조로 되있다.
 */
fn main() {
    let lock0 = Arc::new(Mutex::new(0));

/*
 * prifri, 2022.11.30:
 * - 내부값은 복사회지 않고 참조카운터만 증가한다.
 */
    let lock1 = lock0.clone();

    let th0 = thread::spawn(move || {
        some_func(lock0);
    });

    let th1 = thread::spawn(move || {
        some_func(lock1);
    });

    th0.join().unwrap();
    th1.join().unwrap();
}
