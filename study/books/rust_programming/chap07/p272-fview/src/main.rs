use std::fs::File;
use std::io::prelude::*;
use std::env;

const BYTES_PER_LINE: usize = 1;

fn main() {
/*
 * PRIFRI, 2022.10.05:
 * - nth()
 *   Option타입을 반환. n번째 위치의 요소를 추출한다.
 * - expect
 *   Option값을 다룰때 사용한다. unwrap()보다 더 쉽다고 생각한다고한다.
 */
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

/*
 * PRIFRI, 2022.10.05:
 * - Err가 반환할때까지 while수행
 * - buffer가 해당파일보다 길 경우 오류를 반환하고 buffer는 미정의 상태가
 *   된다고 한다. (이러면 무조건 한바이트씩만 읽어야되는데 ..;;)
 */
    while let Ok(_) = f.read_exact(&mut buffer) {
        if pos % 16 == 0 {
            println!("");
            print!("[0x{:08x}] ", pos);
        }
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte)
            }
        }
        
        pos += BYTES_PER_LINE;
    }

    println!("");
}
