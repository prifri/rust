fn main() {
    let a: i32 = 10;
    let b: Box<i32> = Box::new(40);
    println!("{}", a + *b);
}
