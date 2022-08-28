use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;


/*
 * IAMROOT, 2022.08.29:
 * - try_into : i32로 변환
 * - unwrap : 성공값 처리. i32로 실제 반환하는 역할.
 */
    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("The is less than on hudered.");
    }
}
