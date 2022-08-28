fn main() {

/*
 * IAMROOT, 2022.08.29:
 * - 흔히 코딩할때 많이 언급되는 실수비교.
 * - 여기선 hex로 보여줘서 실제 넘어갈수도있다는걸 보여준다.
 */
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("0.3: {:x}\n", (abc.2).to_bits());
    println!("xyz (f64)");
    println!("0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("0.3: {:x}\n", (xyz.2).to_bits());

    if ((abc.0 + abc.1) == abc.2)
    {
        println!("abc sucees");
    } else
    {
        println!("abc fail");
    }

    if ((xyz.0 + xyz.1) == xyz.2)
    {
        println!("xyz sucees");
    } else
    {
        println!("xyz fail");
    }

/*
 * IAMROOT, 2022.08.29:
 * - rust에서는 실수 비교시 적당히 비교하게 하는 기능이 있다.
 */
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absoulte_difference = (desired - result).abs();
    if (absoulte_difference <= f32::EPSILON)
    {
        println!("abs success");
    } else
    {
        println!("abs fail");
    }

    let x = (-42.0_f32).sqrt();

/*
 * IAMROOT, 2022.08.29:
 * - 음수의 root는 invalid이고 rust에서는 NAN으로 표시한다.
 * - NAN이 있어도 NAN != NAN이다.
 */
    if (x == x)
    {
        println!("x == x success");
    } else
    {
        println!("x == x fail");
    }

/*
 * IAMROOT, 2022.08.29:
 * - 수학적 원인을 확인할수있는 기능.
 */
    let xx: f32 = 1.0 / 0.0;
    if (x.is_finite())
    {
        println!("x is finite");
    } else
    {
        println!("x is not finite");
    }
}
