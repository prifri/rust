use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:10001").await.unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("accept: {}", addr);

        tokio::spawn(async move {
            let (r, w) = socket.split();
            let mut reader = io::BufReader::new(r);
            let mut writer = io::BufWriter::new(w);

            let mut line = String::new();
            loop {
/*
 * prifri, 2022.12.12:
 * - tokio의 read_line 함수는 인수에 전달한 문자열의 끝에 읽은 문자열이
 *   추가되므로 문자열을 초기화.
 */
                line.clear();
/*
 * prifri, 2022.12.12:
 * - 1행 읽기를 비동기실행. 이 직전까지는 동기실행이다.
 */
                match reader.read_line(&mut line).await {
/*
 * prifri, 2022.12.12:
 * - connection close
 */
                    Ok(0) => {
                        println!("closed: {}", addr);
                        return;
                    }
                    Ok(_) => {
                        print!("read: {}, {}", addr, line);
                        writer.write_all(line.as_bytes()).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                    Err(e) => {
                        println!("error: {}, {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}
