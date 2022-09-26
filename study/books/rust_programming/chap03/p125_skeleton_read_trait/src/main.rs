#![allow(unused_variables)]
#[derive(Debug)]
struct File;


/*
 * prifri, 2022.09.27:
 * - trait block은 구현체가 반드시 따라야 할 함수의 시그니처 타입을 포함한다.
 * 의사(pseudo) 타입 Self는 read를 구현하는 타입에 대한 자리 표시자이다.
 */
trait Read {
    fn read(
        self: &Self, save_to: &mut Vec<u8>
        ) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {

/*
 * prifri, 2022.09.27:
 * - 필요한 타입 시그너처를 준수하는 단순 스터브값
 */
        Ok(0)
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec!();
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}
