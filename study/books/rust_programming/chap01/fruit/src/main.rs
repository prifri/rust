fn main() {
    let fruit = vec!['!', '@', '#'];

/*
 * IAMROOT, 2022.08.28:
 * - overflow
 */
    let buffer_overflow = fruit[4];

/*
 * IAMROOT, 2022.08.28:
 * - 두 인자가 같은지 확인.
 */
    assert_eq!(buffer_overflow, '%');
}
