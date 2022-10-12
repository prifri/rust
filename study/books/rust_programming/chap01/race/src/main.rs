use std::thread;

/*
 * prifri, 2022.08.28:
 * - rust는 여러 thread가 한 데이터를 접근하는것을 허용 안한다.
 */
fn main() {
    let mut data = 100;

    thread::spawn(|| {data = 500;});
    thread::spawn(|| {data = 1000;});
    println!("{}", data);
}
