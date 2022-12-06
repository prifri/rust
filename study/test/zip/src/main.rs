/*
 * prifri, 2022.12.05:
 * - https://www.educba.com/rust-zip/
 */

fn test3() {
/*
 * prifri, 2022.12.05:
 * - 제일 적은것 기준으로까지만 iter돈다.
 */
    println!("test3");
    let itr1 = [100, 200, 300, 400, 500, 600];
    let itr2 = [10 , 20];
    //let itr2 = [];
    let itr3 = [1, 2, 3, 4 ,5 , 6];
    let iter = itr1.iter()
        .zip( itr2.iter())
        .zip( itr3.iter())
        .map(|((x, y), z)| x + y + z);
    println!("{:?}", iter);
    for a in iter {
        println!("{}", a);
    }
}

fn test2() {
    println!("test2");
    let itr1 = [100, 200, 300, 400, 500, 600];
    let itr2 = [10 , 20, 30, 40, 50 ,60];
    let itr3 = [1, 2, 3, 4 ,5 , 6];
    let iter = itr1.iter()
        .zip( itr2.iter())
        .zip( itr3.iter())
        .map(|((x, _y), _z)| x);
    println!("{:?}", iter);
    for itr1 in iter {
        println!("{}", itr1);
    }
}

fn test1() {
    println!("test1");
    let itr1 = [100, 200, 300, 400, 500, 600];
    let itr2 = [10 , 20, 30, 40, 50 ,60];
    let itr3 = [1, 2, 3, 4 ,5 , 6];
    let iter = itr1.iter()
        .zip( itr2.iter())
        .zip( itr3.iter())
        .map(|((x, y), z)| (x, y, z));
    println!("{:?}", iter);
    for (itr1, itr2, itr3) in iter {
        println!("{} {} {}", itr1, itr2, itr3);
    }
}

fn main() {
    test1();
    test2();
    test3();
}
