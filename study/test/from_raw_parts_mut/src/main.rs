fn main() {
    let a: usize = 10;
    let ptr = &a as *const usize as *mut u8;
    let val = unsafe {
        std::slice::from_raw_parts_mut(ptr, std::mem::size_of_val(&a))
    };

    println!("{} {:p} {:p}", a, ptr, val);
}
