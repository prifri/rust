fn mock_rand(n: u8) -> f32 {
/*
 * prifri, 2022.10.04:
 * - 0.5 ~ 0.998를 가진 지수부. 0.5 ~ 0.998의 값을 가진다.
 * - float 형식때문에 0.5부터 시작할수밖에없다.
 * - 0.5 + 0.25 + 0.125 ..의 패턴으로 0.998까지 가질수있다는것.
 * - n값을 << 15로 밀어 값을 만든다.
 * - 기본값이 0.5이므로 0.5를 빼주면 0 ~ 약 0.5의 값을 가진다.
 *   여기서 곱하기 2를 해주면 0 ~ 0.998의 값을 가질수잇다.
 *                              | n    |       
 */
    let base: u32 = 0b001111110_00000000_000000000000000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * ( m - 0.5 )
}

fn main() {
    let v: u8 = 0xff;
    println!("max of input range: {:08b} -> {:?}",
             v, mock_rand(v));
    let v: u8 = 0x7f;
    println!("max of input range: {:08b} -> {:?}",
             v, mock_rand(v));
    let v: u8 = 0x00;
    println!("max of input range: {:08b} -> {:?}",
             v, mock_rand(v));
    let v: u8 = 0x01;
    println!("max of input range: {:08b} -> {:?}",
             v, mock_rand(v));
}
