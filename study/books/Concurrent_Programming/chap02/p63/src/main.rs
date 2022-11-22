
/*
 * prifri, 2022.11.22:
 * - closer가 heap상에 생성
 */
fn mul_x(x: u64) -> Box::<dyn Fn(u64) -> u64> {
    let ret = Box::new(move |y| x * y);
/*
 * prifri, 2022.11.22:
 * - y의 소유권이 Box로 옮겨가서 error.
 */
    //println!("y {}", y);
    println!("x {}", x);
    ret
}

fn main() {
/*
 * prifri, 2022.11.22:
 * - y * 3이라는 closer가 f로 생성됨.
 * - 
 * f(y) {
 *  y * 3
 * }
 */
    let f = mul_x(3);
    println!("f(5) = {}", f(5));
}
