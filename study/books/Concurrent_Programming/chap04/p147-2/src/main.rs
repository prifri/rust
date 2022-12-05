use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
/*
 * prifri, 2022.12.05:
 * - _는 참조직후 폐기되므로 readlock 해제가 바로 된다.
 */
        let _ = val.read().unwrap();
        *val.write().unwrap() = false;
        println!("not deadlock");
    });

    t.join().unwrap();
}
