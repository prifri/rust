fn is_even(n: i32) -> bool {
    n & 1 == 0
}

fn main() {
    let n = 12345;
    let desc = if is_even(n) {
        "true"
    } else
    {
        "false"
    };
    println!("{} is {}", n, desc);


    let desc = match is_even(n) {
        true => "true",
        false => "false"
    };

    println!("{} is {}", n, desc);

    let n = loop {
        break 123;
    };

    println!("{}", n);

    let item = 10;
    let test = match item {
        0 => 1,
        10 ..= 20 => 2,
        40 | 80 => 3,
        _ => 4,
    };
    println!("{}", test);
}
