
/*
 * prifri, 2022.10.01:
 * - //!은 현재 항목, 즉 컴파일러가 이제 막 분석을 시작한 모듈을 ㅊ마조한다.
 */
//! 한 번에 한 단계식 파일을 시뮬레이트한다.


/*
 * prifri, 2022.10.01:
 * - ///
 * 바로 뒤에 오는 것에 대한 주석
 */
/// 아마도 파일 시슽메이 있을
/// '파일'을ㅇ 나타낸다
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    /// 새 파일은 비어 있다고 가정하지만 이름은 필요하다.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    /// 파일 길이를 반환한다.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 파일 이름을 반환한다.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let f1 = File::new("f1.txt");

    let f1_name = f1.name();
    let f1_length = f1.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
