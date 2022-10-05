fn main() {
    println!("Hello, world!");
    another_function(10, 20);
}

fn another_function(x: i32, y: i32) {
    println!("x {}, y{}", x, y);
    //let x = five();
    println!("five {} {}",five(), add(five()));
}

fn five() -> i32 {
    5
}

fn add(x : i32) -> i32 {
    x + 1
}
