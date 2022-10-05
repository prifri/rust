use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;
/*
 * PRIFRI, 2022.10.05:
 * - 여러 줄의 문자열을 원시 문자열 리터럴(r 접두사와 #구분자)를 통해 만들면,
 * 큰 따옴표로 이스케이프할 필요가 없다.
 * - 추가적인 b접두사는 이 문자열이 UTF-8텍스트(&str)가 아니고 byte(&[u8])로
 * 취급되어야 함을 지시한다.
 *
 * - 'static
 *   static lifetime을 의미한다. program이 동작중일대 계속 산다.
 * - &[u8]
 *   char *의 의미. char *str = "abc"; 의 의미.
 * - &'static [u8]
 *   &[u8] + 'static lifetime을 의미한다.
 * - char *str = "abc"로 리터럴 스트링을 저장하니 const로 해주는게 명목상
 * 정확하다.
 */
const INPUT: &'static [u8] = br#"
fn main() {
    println!("hello, world!");
}"#;

const VALUE: u64 = 0;
fn add_1(v: &mut u64) {
    *v += 1;
}

/*
 * PRIFRI, 2022.10.05:
 * - const를 *mut를 가져오면 변수값이 생긴다.
 * - add_1함수에서 마치 전역 VALUE를 수정하는것처럼 보이지만 실제론 변수값이
 * 새로 생겨 하나의 지역변수가 생기게 된다
 *
 * - int v = VALUE;
 *   add_1(&v);
 *   add_1(&v);
 *   와 동일하게 동작.
 *
 * - 그래서 const를 mut로 강제로 변경해서 처리하면 동작은 하지만 warning이
 *   발생한다.
 */
fn test()
{
    let v = &mut VALUE;
    add_1(v);
    add_1(v);
    println!("test {} {}", v, VALUE);
}

fn test2() -> std::io::Result<()> {
    let mut abc: &[u8] = br#"
    fn main() {
        println!("hello, world!");
    }"#;
    let mut buffer: Vec<u8> = Vec::with_capacity(abc.len());
    abc.read_to_end (&mut buffer)?;

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}]", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    test();
    test2();
    let mut buffer: Vec<u8> = Vec::with_capacity(INPUT.len());
    INPUT.read_to_end (&mut buffer)?;

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}]", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
