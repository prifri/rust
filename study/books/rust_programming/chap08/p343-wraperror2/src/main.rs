use std::io;
use std::fmt;
use std::net;
use std::fs::File;
use std::net::Ipv6Addr;
use std::error;
/*
 * PRIFRI, 2022.10.06:
 * - from을 구현한다는게 정확히 뭘 의미하고 어떻게 동작하는지 좀 더
 *   설명해줬으면 좋겠다.
 * - from을 구현하면 ?으로 인해 UpstreamError으로 갈시 자동으로
 *   From::UpstreamError 뭐 이런식으로 먼저 찾나보다.
 * - ?은 try!의 매크로문이라고 한다.
 *   https://doc.rust-lang.org/std/macro.try.html
 */
#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError {}

impl From<io::Error> for UpstreamError {
    fn from(error :io::Error) -> Self {
        UpstreamError::IO(error)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(error :net::AddrParseError) -> Self {
        UpstreamError::Parsing(error)
    }
}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
