static B: [u8; 3] = [1, 2, 3];
static C: [u8; 4] = [4, 5, 6, 7];

fn main() {
    let a = 42;
    let b = &B;
    let c = &C;

    println!("{} {:p} {:p}", a, b, c);
}
