use std::{thread, time};

fn main() {
    let pause = time::Duration::from_millis(20);

/*
 * prifri, 2022.10.09:
 * - move는 copy에 의존한다.
 */
    let handle1 = thread::spawn(move || {
        thread::sleep(pause);
    });

    let handle2 = thread::spawn(move || {
        thread::sleep(pause);
    });

    match handle1.join() {
        Ok(_) => (),
        Err(_) => (),
    }
    match handle2.join() {
        Ok(_) => (),
        Err(_) => (),
    }
}
