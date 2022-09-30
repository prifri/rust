#![allow(dead_code)]

#[derive(Debug,PartialEq)]

/*
 * prifri, 2022.10.01:
 * - 전체적인 type을 공개하면 안에값들도 다 공개
 */
pub enum FileState {
    Open,
    Closed,
}


#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl File {

/*
 * prifri, 2022.10.01:
 * - file이 공개됬어도, method가 자동으로 공개되는건 아니다. method마다
 * pub를 붙여줘야 공개된다.
 */
    pub fn new(name: &str) -> File {
        File {
            name : String::from(name),
            data : Vec::new(),
            state : FileState::Closed
        }
    }
}

fn main() {
    let f7 = File::new("f7.txt");
    //...
    println!("{:?}", f7);
}
