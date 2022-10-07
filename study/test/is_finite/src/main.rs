fn main() {
    let num: f64 = 0.0;
    let num2: f64 = f64::INFINITY;
    let num3: f64 = f64::NAN;
    let num4: f64 = 999999.0;

    println!("0: {} infi {} nan {} 999..0{}", num.is_finite(), num2.is_finite(), num3.is_finite(), num4.is_finite());
}
