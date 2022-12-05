mod banker;

use banker::Banker;
use std::thread;

const NUM_LOOP: usize = 1000;
const RESOURCE_CNT: usize = 3;
const THREAD_CNT: usize = 4;
const DEFAULT_AVAILABLE: usize = 1;

fn main() {
    let banker = Banker::<RESOURCE_CNT, THREAD_CNT>
        ::new([[DEFAULT_AVAILABLE; RESOURCE_CNT]; THREAD_CNT]);
/*
 * prifri, 2022.12.05:
 * - array로 초기화하는법은 못찾았다.
 */
    let mut philosophers: Vec<_> = Vec::with_capacity(THREAD_CNT);

    for tidx in 0..THREAD_CNT {
        let banker = banker.clone();
        let p = thread::spawn(move || {
/*
 * prifri, 2022.12.05:
 * - resource를 전부 take 할때까지 while을 돈다
 * - resource를 전부 획득하고 작업을 수행한다.
 * - 작업이 끝낫으면 리소스를 반환한다.
 *
 * - resource에 대한 lock들을 순서대로 획득한다. 전부 획득할때까지
 *   잠길것이다.
 */
            for _ in 0..NUM_LOOP {
                for ridx in 0..RESOURCE_CNT {
                    while !banker.take(tidx, ridx) {}
                }

                //thread::sleep_ms(10);
                //println!("{}: eating", tidx);

                for ridx in 0..RESOURCE_CNT {
                    banker.release(tidx, ridx);
                }
                //thread::sleep_ms(1);
            }
        });

        philosophers.push(p);
    }

    for p in philosophers {
        p.join().unwrap();
    }
}
