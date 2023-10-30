fn main() {
    let v_i32: i32 = 10;
    let v_i64: i64 = 10;
    let v_usize: usize = 10;
    let v_i128: i128 = 10;
    let v_string: String = "abcdefgaaaaaaaaaaaaa".to_string();
    let v_str: &str  = "abc";
    println!("i32 {}, i64 {}, usize {}, i128 {} String {} &str {}",
             std::mem::size_of_val(&v_i32),
             std::mem::size_of_val(&v_i64),
             std::mem::size_of_val(&v_usize),
             std::mem::size_of_val(&v_i128),
             std::mem::size_of_val(&v_string),
             std::mem::size_of_val(&v_str),
             );
}
