fn test_option(a: u32) -> Option<u32> {
    Some(a)
}

fn test_result(a: u32) -> Result<u32, String> {
    Ok(a)
}

fn main() {
    let a = match test_option(10) {
        Some(e) => e,
        None => 10,
    };

    println!("{}", a);

    let a = test_option(10).unwrap();
    println!("{}", a);

    let a = test_result(10).unwrap();
    println!("{}", a);
}
