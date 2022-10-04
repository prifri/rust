fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
/*
 * PRIFRI, 2022.10.04:
 * - i64로 변환. c적으로 intptr_t 변환과 같을듯.
 * uint64_t *p = ...;
 * int64_t addr =  (uintptr_t)p;
 */
    let a_addr: usize = unsafe {
        std::mem::transmute(a_ptr)
    };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
}
