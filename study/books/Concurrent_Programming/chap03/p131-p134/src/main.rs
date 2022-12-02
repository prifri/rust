/*
 * prifri, 2022.12.02:
 * - https://stackoverflow.com/questions/55171692/use-a-module-inside-of-a-module-in-rust
 * crate, mod 사용법 연습
 */

pub mod channels;
/*
 * prifri, 2022.12.02:
 * - module에서 사용하기 위해 main.rs에서 선언을 해줘야한다.
 *   이게 싫으면 channels dir을 만들고 거기서 inner mod로 해줘야 될거같다.
 */
pub mod semaphore;

use channels::channel;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 8;

fn main() {
/*
 * prifri, 2022.12.02:
 * - channel 4개짜리 tx, rx를 만든다.
 */
    let (tx, rx) = channel(4);
    let mut v = Vec::new();

/*
 * prifri, 2022.12.02:
 * - loop * NUM_THREADS만큼 recv를 받는 recv thread를 한개 생선한다.
 */
    let t = std::thread::spawn(move || {
        let mut cnt = 0;
        while cnt < NUM_THREADS * NUM_LOOP {
            let n = rx.recv();
            println!("recv: n = {:?}", n);
            cnt += 1;
        }
    });

    v.push(t);

/*
 * prifri, 2022.12.02:
 * - NUM_THREADS개의 send thread를 만든다.
 */
    for i in 0..NUM_THREADS {
        let tx0 = tx.clone();
        let t = std::thread::spawn(move || {
            for j in 0..NUM_LOOP {
                tx0.send((i, j));
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}
