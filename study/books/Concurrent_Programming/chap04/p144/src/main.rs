use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let c0 = Arc::new(Mutex::new(()));
    let c1 = Arc::new(Mutex::new(()));

    let c0_p0 = c0.clone();
    let c1_p0 = c1.clone();

    let p0 = thread::spawn(move || {
        for _ in 0..100000 {
            let _n1 = c0_p0.lock().unwrap();
            let _n2 = c1_p0.lock().unwrap();
            println!("0: eating");
        }
    });

    let p1 = thread::spawn(move || {
        for _ in 0..100000 {
/*
 * prifri, 2022.12.02:
 * - p0와 같은순서로 lock획득하면 deadlock 안걸린다.
 * ex)
            let _n2 = c0.lock().unwrap();
            let _n1 = c1.lock().unwrap();
 */
            let _n1 = c1.lock().unwrap();
            let _n2 = c0.lock().unwrap();
            println!("1: eating");
        }
    });

    p0.join().unwrap();
    p1.join().unwrap();
}
