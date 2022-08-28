fn main() {
    let three = 0b11;
    let thiry = 0o36;
    let three_hudred = 0x12C;

/*
 * IAMROOT, 2022.08.28:
 * - binary, octal, hexadecimal
 */
    println!("base 10: {} {} {}", three, thiry, three_hudred);
    println!("base 2: {:b} {:b} {:b}", three, thiry, three_hudred);
    println!("base 8: {:o} {:o} {:o}", three, thiry, three_hudred);
    println!("base 2: {:x} {:x} {:x}", three, thiry, three_hudred);
}
