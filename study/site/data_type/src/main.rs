fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("array {}", a[0]);

    // compile error
    //println!("array {}", a[5]);

    //let idx = 10;
    //println!("array {}", a[idx]);

    //let ele = a[idx];
    //println!("a {}", ele);
    
    let b: (i32, f64, u8) = (500, 6.4, 1);

    println!("{} {} {}", b.0, b.1, b.2);

    let (x, y, z) = b;

    println!("{} {} {}", x, y, z);
}
