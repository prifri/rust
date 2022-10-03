
/*
 * prifri, 2022.10.03:
 * - compiler가 오버플로 상황을 검출할수있게 한다.
 */
#[allow(arithmetic_overflow)]

fn main() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("{} + {} = {}", a, b, c);
}
