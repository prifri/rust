use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
/*
 * IAMROOT, 2022.12.05:
 * - read lock걸리고 write lock을 얻을려하니 deadlock
 */
        let _flag = val.read().unwrap();
        *val.write().unwrap() = false;
        println!("deadlock");
    });
    t.join().unwrap();
}
