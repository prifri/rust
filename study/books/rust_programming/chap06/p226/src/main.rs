/*
 * PRIFRI, 2022.10.04:
 * - 반드시 필요할때에만 복사가 일어나는 포인타.
 * - 포인터 주소로부터 읽는 스마트 포인터 타입
 */
use std::borrow::Cow;
/*
 * PRIFRI, 2022.10.04:
 * - rust가 '\0'를 읽을수 있도록 한다
 */
use std::ffi::CStr;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115,104, 0];

fn test2() {
    use std::mem;

    unsafe {
        let s = String::from("hello");

        // Prevent automatically dropping the String's data
        let mut s = mem::ManuallyDrop::new(s);

        let ptr = s.as_mut_ptr();
        let len = s.len();
        let capacity = s.capacity();

        let s = String::from_raw_parts(ptr, len, capacity);

        assert_eq!(String::from("hello"), s);
    }
}

fn test() {
    let a = 42;
/*
 * PRIFRI, 2022.10.04:
 * - 스마트 포인터
 *   type과 length를 header에 저장한 pointer 개념
 * - String
 *   스마트 포인터 타입. 
 */
    let b: String;
    let c: Cow<str>;

    unsafe {
/*
 * PRIFRI, 2022.10.04:
 * - from_raw_parts에서 *mut가 필요로 한 상황인데, 참조는 *mut로 바로 변환이
 *   안된다. 하지만 *const -> *mut는 가능해서 이렇게 한다고 한다.
 * - from_raw_parts
 *   ptr, 용량, 크기 에대한 값을 받는다.
 * - B는 '\0'이 없는 문자열이므로 고정크기로 받는다.
 *   array type을 pointer로 변환후 가변 포인터로 변환을 해서 쓰는거같다.
 * - unsafe에서 from_raw_parts를 이런식으로 사용하면 core dump(invalid free)가
 *   발생하는듯 싶다. test2의 예제처럼 써야되거나 box등을 사요해야되는듯.
 */
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
/*
 * PRIFRI, 2022.10.04:
 * - asckii 표준을 따르므로 u8->i8로 변환. c_char은 i8을 가리킨다.
 */
        let c_ptr = &C as *const u8 as *const c_char;
/*
 * PRIFRI, 2022.10.04:
 * - from_ptr을 통해서 '\0'까지 읽으면서 string을 c에 저장한다.
 */
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn main() {
    test2();
    test();
}
