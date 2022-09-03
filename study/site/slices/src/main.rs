fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    sadfasdf


    for (i, &item) in bytes.iter().enumerate() {
        print!("i {} item {} ", i, item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn exampe() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {} {}", s, hello, world);
}

fn main() {
    println!("Hello, world!");

    let s = String::from("ab c");

    //s.clear();
    println!("result {}", first_word(&s));

    exampe();
}
