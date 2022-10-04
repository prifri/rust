fn main() {
    let a: i64 = 42;
/*
 * PRIFRI, 2022.10.04:
 * - 원시 포인터
 */
    let a_ptr: *const i64 = &a;
    println!("a: {} ({:p})", a, a_ptr);
}
