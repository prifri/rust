
/*
 * prifri, 2022.10.11:
 * - std::sync::mpsc
 *   rust 표준 라이브러리. 여긴 사용 안한다.
 * - crossbeam
 *   mpsc보다 더 낫다고 한다.
 *   crossbeam으로 채널을 만들려면 Sender<T>와 Receiver<T>를 반환하는 함수를
 *   호출해야 한다.
 */
/*
 * prifri, 2022.10.11:
 * - select! 매크로를 사용할수 있다.
 */
#[macro_use]
extern crate crossbeam;

use std::thread;
use crossbeam::channel::unbounded;

fn main() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(42)
            .unwrap();
    });

/*
 * prifri, 2022.10.11:
 * - 메인 스레드가 메시지를 차단하고 기다릴 수 있게 해 준다.
 */
    select!{
        recv(rx) -> msg => println!("{:?}", msg),
    }
}
