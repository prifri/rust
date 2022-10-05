#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn test1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    if let four = &home.kind
    {
        println!("{} {}", home.address, loopback.address);
    }

    if let six = &home.kind
    {
        println!("{} {}", home.address, loopback.address);
    }
}

enum IpAddr2 {
    V4(String),
    V6(String)
}

fn test2() {
    let home = IpAddr2::V4(String::from("128.1.1"));
    let loopback = IpAddr2::V6(String::from("1::1"));

    if let home = IpAddr2::V4
    {
        println!("home is v4");
    } else if let home = IpAddr2::V6
    {
        println!("home is v6");
    }

}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn test3() {
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test4() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    let absent_number = Option::<i32>:: None;
    let six = plus_one(some_number);
    let none = plus_one(Option::None);

    if let six = Option::Some(5) {
        println!("option six is six");
    }
}

enum UsSate {
    Alababa,
    ABcd,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Abcdef(UsSate)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Quarter => 10,
        Coin::Abcdef(state) => {
            //println!("State quater from {:?}!", state);
            25
        },
        _ => 0
    }

}

fn test5()
{
    //println!("{}", value_in_cents(&Coin::Penny));
    let a = Coin::Dime;

    println!("{}", value_in_cents(a));
    println!("{}", value_in_cents(Coin::Abcdef(UsSate::ABcd)));

    let some_u8_value = 1u8;

    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}
