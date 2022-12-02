/*
 * prifri, 2022.12.01:
 * - cargo에 lib로 정의해서 mod 선언이 필요없는 상태.
 */
use semaphore::Semaphore;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 8;
const SEM_NUM: isize = 4;

static mut CNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut v = Vec::new();
    v.reserve(SEM_NUM as usize);
    let sem = Arc::new(Semaphore::new(SEM_NUM));

    for i in 0..NUM_THREADS {
        let s = sem.clone();

        let t= std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                s.wait();

/*
 * prifri, 2022.12.01:
 * - Ordering::SeqCst
 *  sequence memory barrier.
 *  요즘 atomic 명령어를 보면 atomic + memory barrier 옵션을 주게 해서 그에 따라 동작하게 한다.
 *  acquire, release, relaxed, sequence등이 있는데
 *  여기선 fetch_add할때와 fetch_sub할때만 s.wait()와 s.post가 안섞이게 sequence를 사용한다.
 *  책에선 load를 써서 atomic read를 했는데, fetch_add 의 return값이 변경전 값을 atomic하게
 *  가져오는거라
 *  이걸 쓰면서 + 1 하면 현재값이 되니 딱히 load를 한번더 안해도된다. atomic read를 할때 load를
 *  쓴다고만 기억하고 있으면 될듯.
 */
                let n;
                unsafe { n = CNT.fetch_add(1, Ordering::SeqCst) };
                println!("semaphore: i = {}, CNT = {}", i, n + 1);
                assert!((n as isize) <= SEM_NUM);
                unsafe { CNT.fetch_sub(1, Ordering::SeqCst) };

                s.post();
            }
        });

        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}
