fn use_value(_val: i32) {
}

fn use_value2(_val: Demo) {
}

struct Demo {
    a: i32,
}

fn main() {
    let a:i32 = 0;

/*
 * prifri, 2022.10.03:
 * - 원시타입은 copy trait가 있다
 * - Copy trait
 * 복제하지 않고서는 사용할수 없을때에 한해 복제 된다.
 * 원시타입은 이미 구현되있다.
 *
 * - Copy trait가 있으면 Copy semantics. 없으면 Move semantics가 발생한다.
 *
 * - Move semantics
 * 소유권이 이전됨을 뜻한다.
 *
 * ---
 * c적으로
 *
 * fn (struct value)
 * {
 *    ...
 * }
 *
 * main()
 * {
 *      struct a = ..;
 *      fn(a);
 * }
 *
 * 이렇게되면 a는 fn에서 새로운 memory에 shadow copy되며 fn에서 value로
 * 사용되고 a는 main에 그대로 남아서 사용될것이다.
 *
 * 하지만 rust에서는 copy가없을경우 a가 fn으로 마치 참조처럼 취급되면서
 * main에서는 접근하지 못하게 하는것처럼 구현한거 같다.
 *
 * 만약 copy trait를 구현했으면 copy trait에서 shadow든 deep 이든 copy
 * 하고싶은대로 새로 copy에서 쓰게되면 main에서 그대로 써지게 되는 개념인듯
 */
    use_value(a);

    println!("{}", a);

    let a = Demo {a: 123};

    use_value2(a);

    println!("{}", a);
}
