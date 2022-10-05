use std::path::PathBuf;

fn main() {
    let mut hello = PathBuf::from("/tmp/hello.txt");
    hello.pop();
    println!("{:?}", hello.display());
}
