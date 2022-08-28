fn main() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_tw = 22_i32;

    let addition = twenty + twenty_one + twenty_tw;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_tw, addition);

/*
 * IAMROOT, 2022.08.28:
 * - 가독성 높임.
 */
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:02.11}", forty_twos[0]);
}
