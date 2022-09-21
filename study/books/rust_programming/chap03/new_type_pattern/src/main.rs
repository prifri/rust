
/*
 * prifri, 2022.09.22:
 * - page 103
 */
struct Hostname(String);

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());
    
    //connect(ordinary_string);
    connect(Hostname(ordinary_string));
}
