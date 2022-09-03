fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    }

    y
}

fn test() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn test1() {
    let string1 = String::from("abcd");

    //let result;
    {
        let string2 = String::from("xyz");
        //result = longest(string1.as_str(), string2.as_str());
    }
    //println!("The longest string is {}", result);
}

fn main() {
    test();
    test1();
}
