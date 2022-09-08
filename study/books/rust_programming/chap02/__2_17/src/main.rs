
/*
 * prifri, 2022.09.08:
 * - trait
 *   interface, protocol, contract과 비슷한 언어 기능이라고 한다.
 *
 * - std::ops에서 Add trait을 지역변수로 가져온다.
 */
use std::ops::Add;
use std::time::Duration;

/*
 * prifri, 2022.09.08:
 * std::ops::Add를 구현해야된다는걸 명시한다.
 */
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    println!("{}", add(1.2, 2.3));
    println!("{}", add(10, 20));
    let durations = add(
        Duration::new(5, 0),
        Duration::new(10, 0)
    );

/*
 * prifri, 2022.09.08:
 * - std::time::Duration이 std::fmt::Display trait을 구현하지 않았으므로
 * std::fmt::Debug를 쓰는 것으로 대신한다.
 */
    println!("{:?}", durations);
}
