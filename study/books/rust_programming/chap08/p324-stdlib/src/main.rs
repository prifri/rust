use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let host = "www.rustinaction.com:80";

    let mut connection = TcpStream::connect(host)?;
/*
 * PRIFRI, 2022.10.06:
 * - HTTP 1.0
 *   서버가 응답을 보내고 disconnect. keep 기능이 없다고 한다.
 * - HTTP 1.1
 *   서버가 응답을 보내고 keep. client에서 disconnect 명령을 줘야되는데
 *   현재 code에서 없으므로 사용안한다.
 */
    connection.write_all(b"GET / HTTP/1.0")?;
/*
 * PRIFRI, 2022.10.06:
 * - \r\n
 *   새줄임을 알린다.
 */
    connection.write_all(b"\r\n")?;
    connection.write_all(b"Host: www.rustinaction.com")?;
/*
 * PRIFRI, 2022.10.06:
 * - 두개의 빈줄은 끝을 알린다.
 */
    connection.write_all(b"\r\n\r\n")?;
/*
 * PRIFRI, 2022.10.06:
 * - Reader -> Writer로 보낸다.
 */
    std::io::copy(
        &mut connection,
        &mut std::io::stdout()
    )?;

    Ok(())
}
