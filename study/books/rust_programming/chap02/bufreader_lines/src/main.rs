use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

/*
 * prifri, 2022.09.21:
 * - BufReader::read_line() : 한줄읽음. 개행까지 읽는듯
 * - BufReader::lines() : 뒤에 개행 삭제해서 던져줌.
 */
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
