use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
/*
 * prifri, 2022.12.05:
 * - read 직후 참조를하고 끝내므로 lock이 해제된다. 값복사가 일어난다.
 */
        let flag = *val.read().unwrap();
        if flag {
            *val.write().unwrap() = false;
            println!("flag is true");
        }
    });

    t.join().unwrap();
}
