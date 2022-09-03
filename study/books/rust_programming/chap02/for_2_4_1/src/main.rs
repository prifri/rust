fn main() {
    let collection = [1, 2, 3, 4, 5];

/*
 * prifri, 2022.09.03:
 * - readonly
 * for 문은 compile 시 검증을 한다.
 */
    for item in &collection {
        print!("{}", item);
    }
    println!("");

    let mut collection = [1, 2, 3, 4, 5];
/*
 * prifri, 2022.09.03:
 * - read/write
 */
    for item in &mut collection {
        print!("{}", item);
        *item = *item + 1;
    }
    println!("");

    for item in &mut collection {
        print!("{}", item);
    }
    println!("");

/*
 * prifri, 2022.09.03:
 * - 문법상 문제 없으나 비효율.
 * collection[i] 접근이 유효한지 runtime시 비용이 있다고한다.
 */
    let collection = [1, 2, 3, 4, 5];
    for i in 00..collection.len() {
        let item = collection[i];
        print!("{}", item);
    }
    println!("");

    let mut i = 0;
/*
 * prifri, 2022.09.03:
 * - exclusive range. 익명 변수.
 */
    for _ in 0..10 {
        i += 1;
    }
    println!("{}", i);

    i = 0;
    for _ in 0..=10 {
        i += 1;
    }
    println!("{}", i);

    i = 0;
    for _ in 5..10 {
        i += 1;
    }
    println!("{}", i);

    let mut i = 0;
    for _ in 0..collection.len() {
        i += 1;
    }
    println!("{}", i);
}
