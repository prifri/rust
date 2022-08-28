fn greet_world() {
/*
 * prifri, 2022.08.28:
 * ! : macro라는 의미
 */
    println!("Hello, world!");
    let southern_germany = "مرحبا بالعالم!";
    let korean = "하나둘셋";
/*
 * IAMROOT, 2022.08.28:
 * - [] : 배열 리터럴.
 */
    let regions = [southern_germany, korean]; //배열 리터널

    for region in regions.iter() {

/*
 * IAMROOT, 2022.08.28:
 * - &region : read only borrow
 */
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}

