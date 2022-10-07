use std::{thread, time};

fn main() {
    let start = time::Instant::now();

/*
 * prifri, 2022.10.08:
 * - move
 *   부모범위에 정의된 변수(capture)를 접근할려면 closer로 옮겨야 한다.
 *   이 소유권을 넘기는걸 나타낼려면 익명 함수에 move란 키워드를 쓴다.
 */
    let handler = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    handler.join().unwrap();

    let finish = time::Instant::now();

    println!("{:02?}", finish.duration_since(start));
}
