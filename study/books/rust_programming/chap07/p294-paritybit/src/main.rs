fn parity_bit(bytes: &[u8]) -> u8 {
    let mut n_ones: u32 = 0;

/*
 * prifri, 2022.10.06:
 * - set bit count
 */
    for byte in bytes {
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b}) has {} one bits", byte, byte, ones);
    }

    (n_ones & 1 == 0) as u8
}

fn main() {
    let abc = b"abc";
    println!("input: {:?}", abc);
    println!("output: {:08x}\n", parity_bit(abc));
    let abcd = b"abcd";
    println!("input: {:?}", abcd);
    println!("result: {:08x}", parity_bit(abcd));
}
