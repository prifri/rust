use std::mem;

fn main() {

/*
 * prifri, 2022.10.03:
 * - 0xDDCCBBAA 라는 숫자가 있다고 할때
 *           31..    0
 *   big    : DDCCBBAA
 *   little : AABBCCDD
 */
    let big_endian: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
    let little_endian: [u8; 4] = [0xdd, 0xcc, 0xbb, 0xaa];

    let a: i32 = unsafe {
        mem::transmute(big_endian)
    };
    let b: i32 = unsafe {
        mem::transmute(little_endian)
    };

    println!("{} {}\n\
    {:x} {:x}\n\
    {:b} {:b}", a, b, a, b, a, b);
}
