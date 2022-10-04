use std::mem::size_of;

static B: [u8; 3] = [1, 2, 3];
static C: [u8; 4] = [4, 5, 6, 7];

fn main() {
    let a: usize = 42;
    let b: &[u8; 3] = &B;
    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size:      {:?} bytes", size_of::<usize>());
    println!("  value:     {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!("  location: {:p}", &b);
    println!("  size:      {:?} bytes", size_of::<&[u8; 3]>());
    println!("  value:     {:?}", b);
    println!();
/*
 * PRIFRI, 2022.10.04:
 * - 길이까지 있기에 8 + 8
 */
    println!("c (a \"box\" for C):");
    println!("  location: {:p}", &c);
    println!("  size:      {:?} bytes", size_of::<Box<[u8]>>());
    println!("  value:     {:?}", c);
    println!();

    println!("B (an array of 3 bytes):");
    println!("  location: {:p}", &B);
    println!("  size:      {:?} bytes", size_of::<&[u8; 3]>());
    println!("  value:     {:?}", B);
    println!();

    println!("C (an array of 4 bytes):");
    println!("  location: {:p}", &C);
    println!("  size:      {:?} bytes", size_of::<&[u8; 4]>());
    println!("  value:     {:?}", C);
    println!();
}
