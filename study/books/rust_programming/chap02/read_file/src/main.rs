use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {

/*
 * prifri, 2022.09.21:
 * - File객체를 만들때 경로 인자가 필요하다. 파일이 존재하지 않으면 오류가 발생한다.
 * unwrap을 통해 강제 종료 시키는듯.
 */
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
/*
 * prifri, 2022.09.21:
 * - open했어도. read 실패가 가능하다.
 */
        let len = reader.read_line(&mut line)
            .unwrap();

        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);

/*
 * prifri, 2022.09.21:
 * - String 객체 길이를 0으로 줄인다.
 */
        line.truncate(0);
    }
}
