
/*
 * prifri, 2022.09.08:
 * - <'a, 'b> : 수명변수 a, 수명변수 b
 * - i: &'a i32 수명변수 'a를 i의 수명으로 바인드한다. 
 * 매개 변수 i는 수명 a를 가지는 i32 타입의 참조다.
 */
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn main() {
    let a = 10;
    let b = 10;

    println!("{}", add_with_lifetimes(&a, &b));
}
