fn main() {
    let mut i: u16 = 0;
    println!("{}..", i);

    let mut cnt = 10000;

/*
 * prifri, 2022.10.03:
 * - 최적화(rustc -O ..)를 하면 경계 검사(bound check)를 하지 않는다.
 */
    loop {
        i += 1000;
        print!("{}..", i);
        if i % 10000 == 0 {
            print!("\n");
        }
        cnt -= 1;

        if cnt == 0
        {
            return
        }
    }
}
