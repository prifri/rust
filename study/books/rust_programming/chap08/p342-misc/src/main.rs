use std::io;
use std::fmt;
use std::net;
use std::fs::File;
use std::net::Ipv6Addr;
/*
 * PRIFRI, 2022.10.06:
 * - 이거 왜 빠졋을까.. p340에는 써져있는데 정작 예제엔 빠져있다.
 */
use std::error;

/*
 * PRIFRI, 2022.10.06:
 * - Debug print할수 있게 정의.
 */
#[derive(Debug)]
/*
 * PRIFRI, 2022.10.06:
 * - 출력될 error들을 열거값으로 정의한다.
 */
enum UpstreamError{
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/*
 * PRIFRI, 2022.10.06:
 * - 컴파일러가 자동으로 코드를 만들어준다고한다.
 *   Debug와 Display를 구현했기때문에 자동으로 만들어준다고 한다.
 *   정확하겐 잘 모르겠다.
 */
impl error::Error for UpstreamError {}

/*
 * PRIFRI, 2022.10.06:
 * - map_err()
 *   오류를 함수에 매핑한다. ?연산자는 끝에 있어야 한다. 그렇지 않으면
 *   코드가 오류를 반환하기전에 함수가 종료될수있다고한다.
 */
fn main() -> Result<(), UpstreamError> {
/*
 * PRIFRI, 2022.10.06:
 * - 해당 error를 열거형으로 정의해서 알려준다. 
 *   이런 느낌인데 열거 타입까지 잇어서 callback func 처럼 동작하는듯..
 * enum error = f();
 *
 * if (error)
 * {
 *   return error
 * }
 * error = f1();
 *
 * if (error)
 * {
 *   return error
 * }
 */
    let _f = File::open("invisible.txt")
        .map_err(UpstreamError::IO)?;

    let _localhost = "::1"
        .parse::<Ipv6Addr>()
        .map_err(UpstreamError::Parsing)?;

    Ok(())
}
