/*
 * prifri, 2022.10.06:
 * - Vec<T>가 파일인듯 흉내 내는 기능이 필요하다고 한다.
 *   io::Cursor는 인메모리 Vec<T>가 파일처럼 보이도록 하는 역할을 한다.
 */
use std::io::Cursor;

/*
 * prifri, 2022.10.06:
 * - read_*, write_* 메서드를 위한 타입 인자로 사용된다.
 */
use byteorder::LittleEndian;

/*
 * prifri, 2022.10.06:
 * - read_*, write_* 메서드를 제공한다.
 */
use byteorder::{ReadBytesExt, WriteBytesExt};

fn write_numbers_to_file() -> (u32, i8, f64) {

/*
 * prifri, 2022.10.06:
 * - w = writer
 *   w가 file에 쓰는것인향 흉내낸다.
 */
    let mut w = vec![];

    let one: u32 = 1;
    let two: i8 = 2;
    let three: f64 = 3.0;

    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", &w);

    w.write_i8(two).unwrap();
    println!("{:?}", &w);

    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &w);

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one_ = r.read_u32::<LittleEndian>().unwrap();
    let two_ = r.read_i8().unwrap();
    let three_ = r.read_f64::<LittleEndian>().unwrap();

    (one_, two_, three_)
}

fn main() {
    let (one, two, three) = write_numbers_to_file();
    let (one_, two_, three_) = read_numbers_from_file();

    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);
}
