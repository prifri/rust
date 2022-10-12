
/*
 * prifri, 2022.08.29:
 * - Complex 단일 타입만을 가져온다.
 */
use num::complex::Complex;

fn main() {

/*
 * prifri, 2022.08.29:
 * - 생성자가 없는 대신 리터럴 타입이 있으며 {}로 타입을 초기화.
 */
    let a = Complex { re: 2.1, im: -1.2};

/*
 * prifri, 2022.08.29:
 * - 간결함을 위해서 new를 사용.
 */
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + i{}", result.re, result.im);
}
