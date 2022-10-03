fn main() {
    let a: f32 = 42.42;

/*
 * prifri, 2022.10.03:
 * - std::mem::transmute
 *   주어진 비트에 영향을 끼치지 않고 f32를 u32로 변환하는것.
 * - unsafe
 *   data type을 변동하는 것은 명확하지 않은 행동인데, 컴파일러한테
 *   개발자가 정확성을 보증하겠다는 걸 알린다.
 */
    let frankentype: u32 = unsafe {
        std::mem::transmute(a)
    };

/*
 * prifri, 2022.10.03:
 * - b
 *   std::fmt::Binary trait를 호출한다.
 * - f32 type은 std::fmt::Debug가 없다. u32로 bit 영향없이 변환해 u32의
 *   Debug를 활용해서 표현한다.
 */
    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };

    println!("{}", b);
    assert_eq!(a, b);
}
