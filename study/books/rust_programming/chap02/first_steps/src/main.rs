
/*
 * prifri, 2022.08.28:
 * - let : read only
 * - let mut : read/write
 */
fn main() {

/*
 * prifri, 2022.08.28:
 * - compiler가 추론하게함.
 */
    let a = 10;

/*
 * prifri, 2022.08.28:
 * - programmer가 직접 정함
 */
    let b: i32 = 20;

/*
 * prifri, 2022.08.28:
 * - type정함.
 */
    let c = 30i32;

/*
 * prifri, 2022.08.28:
 * - _ : 가독성을 좋게 할뿐 기능은 같음
 */
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

/*
 * prifri, 2022.08.28:
 * - {} : C의 %s
 */
    println!("(a + b ) + (c + d) = {}", e);
}

/*
 * prifri, 2022.08.28:
 * - 함수 정의에는 반드시 타입이 있어야된다.
 * - -> : return type.
 * int32_t add (int i, int j)
 * {
 *  return i + j;
 * }
 */
fn add(i: i32, j: i32) -> i32 {
    i + j
}
