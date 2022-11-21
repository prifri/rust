/*
 * prifri, 2022.11.21:
 * - rust에서 함수 포인터를 써본적이없다. 연습삼아서 해봤다.
 */
fn app_n(f: fn(u64) -> u64, mut n: u64, mut x: u64) -> u64 {
    loop {
        if n == 0 {
            return x;
        }
        x = f(x);
        n -= 1;
    }
}

fn mul2(x: u64) -> u64 {
    x * 2
}

fn main() {
    println!("app_n(mul2, 4, 3) = {}", app_n(mul2, 4, 3));
}
