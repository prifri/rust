fn main() {
    let one = [1, 2, 3];

/*
 * prifri, 2022.09.08:
 * - [type; count] 세미콜론으로 구분된 두값을 입력하는 반복 표현식.
 */
    let two: [u8; 3] = [1, 2, 3];

/*
 * prifri, 2022.09.08:
 * - [0; 3] = [0, 0, 0]
 */
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

/*
 * prifri, 2022.09.08:
 * - 이차원 배열. 3X4
 */
    let arrays = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }

        println!("\t(sigma{:?} = {})", a, sum);
    }
}
