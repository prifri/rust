use std::collections::HashMap;

fn main() {
    let mut capitals = HashMap::new();

    capitals.insert("Cook Islands", "Avarua");
    capitals.insert("AA", "11");
    capitals.insert("BB", "12");
    capitals.insert("CC", "13");
    capitals.insert("DD", "14");
    capitals.insert("EE", "15");

    let tongan_capital = capitals["DD"];
    println!("Capital of DD is: {}", tongan_capital);
}
