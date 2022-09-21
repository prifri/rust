
/*
 * prifri, 2022.09.22:
 * - println!으로 file을 출력할 수 있도록 한다.
 * std::fmt::Debug trate는 매크로 내에서
 * {:?}과 연계하여 file을 출력 가능한 문자열로 바꾼다.
 */
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {

/*
 * prifri, 2022.09.22:
 * - String::from은 slice인 무자열 리터럴에서 소유한 무자열을 생성한다.
 */
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
