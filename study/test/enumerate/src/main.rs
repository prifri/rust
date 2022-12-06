fn main() {
    let iter1 = vec![1, 2, 3, 4];
    for (i, m) in iter1.iter().enumerate() {
        println!("{} {}", i, m);
    }

    let iter1 = vec![10, 11, 12, 13];
    let iter2 = vec![5, 6, 7, 8];
    for (m, n) in iter1.iter().zip(iter2.iter()) {
        println!("{} {}", m, n);
    }

    let iter1 = vec![10, 11, 12, 13];
    let iter2 = vec![5, 6, 7, 8];
    for (i, (m, n)) in iter1.iter().zip(iter2.iter()).enumerate() {
        println!("{} {} {}", i, m, n);
    }

    let iter1 = vec![10, 11, 12, 13];
    let iter2 = vec![5, 6, 7, 8];
    for (i, (m, n)) in iter1.iter().zip(iter2.iter()).enumerate() {
        println!("{} {} {}", i, m, n);
    }

    let cal = iter1.iter().zip(iter2.iter()).map(|(x, y)| x + y);

    for (i, c) in cal.enumerate() {
        println!("{} {}", i, c);
    }

    let check = iter1.iter().zip(iter2.iter()).all(|(x, y)| *x >= 10 && *y >= 5);
    println!("{}", check);

    let check = iter1.iter().zip(iter2.iter()).all(|(x, y)| *x >= 10 && *y == 5);
    println!("{}", check);
}
