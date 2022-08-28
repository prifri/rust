
/*
 * IAMROOT, 2022.08.28:
 * - Cereal 열거형을 출력할때 println! 매크로를 사용할수 있도록 한다.
 */
#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Weat,
}

fn main() {

/*
 * IAMROOT, 2022.08.28:
 * - 빈 vector 정의
 */
    let mut grains: Vec<Cereal> = vec![];

/*
 * IAMROOT, 2022.08.28:
 * - Rye push
 */
    grains.push(Cereal::Rye);

/*
 * IAMROOT, 2022.08.28:
 * - grains 삭제.
 */
    drop(grains);

/*
 * IAMROOT, 2022.08.28:
 * - 삭제된 grains에 접근. bug.
 */
    println!("{:?}", grains);
}
