
/*
 * prifri, 2022.09.30:
 * - 사용하지 않은(FileSatte::Open) 경고를 출력하지 않게 한다.
 */
#![allow(dead_code)]

/*
 * prifri, 2022.09.30:
 * - fmt::Result를 사용할수 있게 한다.
 */
use std::fmt;

/*
 * prifri, 2022.09.30:
 * - fmt::Display를 Display로 사용할수 있게 한다.
 */
use std::fmt::Display;

#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}


/*
 * prifri, 2022.09.30:
 * - state를 {}표시.
 */
impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {

/*
 * prifri, 2022.09.30:
 * - wrtie!를 사용해서 fmt::Result를 return하게 한다.
 * - fmt::Result
 * Display의 return 값.
 */
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}


/*
 * prifri, 2022.09.30:
 * - file읠 {}로 표시
 */
impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>",
        self.name, self.state)
    }

}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let f6 = File::new("f6.txt");

/*
 * prifri, 2022.09.30:
 * - Debug구현에 따른다.
 */
    println!("1. {:?}", f6);

/*
 * prifri, 2022.09.30:
 * - 위에서 구현한 Display for File에 따른다.
 */
    println!("2. {}", f6);
}
