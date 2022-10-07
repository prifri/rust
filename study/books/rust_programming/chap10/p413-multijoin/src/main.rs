use std::{thread, time};

fn main() {
    for n in 1..=1000 {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

        let start = time::Instant::now();
        for _m in 0..=n {
            let handle = thread::spawn(|| {
                let pause = time::Duration::from_millis(20);
                thread::sleep(pause);
            });
            handlers.push(handle);
        }

        let mut done_cnt = 0;
        let mut fail_cnt = 0;

        while let Some(handle) = handlers.pop() {
            match handle.join() {
                Ok(_) => done_cnt += 1,
                Err(_) => fail_cnt += 1,
            }
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?} {} {}", n,
                 finish.duration_since(start), done_cnt, fail_cnt);
    }
}
