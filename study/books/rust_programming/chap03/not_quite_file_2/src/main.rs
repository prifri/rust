
/*
 * prifri, 2022.09.22:
 * - 104p
 */
#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(
    f: &File,
    save_to: &mut Vec<u8>
    ) -> usize {

/*
 * prifri, 2022.09.22:
 * - append를 하면 mov의 개념이 되서 미리 복사본을 만들어놓는다.
 */
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

/*
 * prifri, 2022.09.22:
 * - read_length만큼 memory를 확장한다. append하면 자동생성
 * 될거같긴한데 모자르면확장, 모자르면확장의 개념으로 반복하는 개념이라
 * 한번에 크게 한번 확장하는개념인듯.
 */
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8>= vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

/*
 * prifri, 2022.09.22:
 * - byte를 string으로 변환. utf8이 아닌건 ?특수문자로 변환한다는듯.
 */
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);
}
