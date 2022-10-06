use std::fs::File;
use std::error::Error;
use std::net::Ipv6Addr;
/*
 * PRIFRI, 2022.10.06:
 * - File open의 trait는 std::io::Error
 *   Ipv6addr parse의 trait는 std::net::AddrParseError.
 * - 서로 다른 type의 error다. 이 경우 ? 만으로 사용할순없다. ?는 단일 error
 *   type만을 사용할수 있기 때문이다.
 * - 그래서 Box<dyn Error>를 오류 타입으로 지정해서 사용한다.
 * - 단점.
 *   어디서 발생했는지 모름.
 */
fn main() ->  Result<(), Box<dyn Error>> {
    let _f = File::open("invisible.txt")?;
/*
 * PRIFRI, 2022.10.06:
 * - 이게 맞는 문법인듯..
 *   https://stackoverflow.com/questions/42030345/how-to-create-an-ipaddr-without-knowing-the-specific-ip-version
 */
    let _localhost: Ipv6Addr = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
