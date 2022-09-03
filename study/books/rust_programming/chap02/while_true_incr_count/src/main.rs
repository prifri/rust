use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    //1초
    let time_liumit = Duration::new(1, 0);
    let start = Instant::now();

    //1초동안 loop를 몇번 도는지 보는것.
    while (Instant::now() - start) < time_liumit {
        count += 1;
    }

    println!("{}", count);
}
