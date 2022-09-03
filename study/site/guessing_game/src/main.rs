extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("secu {}", secret_number);

    loop {
        println!("Plz in num.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("u guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
