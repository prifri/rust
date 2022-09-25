
/*
 * prifri, 2022.09.25:
 * - rand crate를 지역범위로 가져온다.
 */
use rand::random;

static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
/*
 * prifri, 2022.09.25:
 * - 정적 가변변수에 접근하는것은 안전하지 않은 방법이라 unsafe.
 * - unsafe
 * C에서 늘 제공하는 정도의 안정성 수준과 동일한 정도
 */
    unsafe {
        if ERROR != 0 {
            panic!("an error has occurred!")
        }
    }
}
