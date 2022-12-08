use nix::sys::epoll:: {
    epoll_create1, epoll_ctl, epoll_wait, EpollCreateFlags, EpollEvent,
    EpollFlags, EpollOp,
};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;
use std::os::unix::io::{AsRawFd, RawFd};

/*
 * prifri, 2022.12.08:
 * - rust epoll
 *   c의 epoll api와 똑같은 구조를 가지는게 보인다.
 * - event-driven(이벤트 주도)
 *   i/o multiplexing의 방법론중하나로, event로 처리하는것.
 *   비동기 프로그래밍으로 간주한다.
 */
fn main() {
    let epoll_in = EpollFlags::EPOLLIN;
    let epoll_add = EpollOp::EpollCtlAdd;
    let epoll_del = EpollOp::EpollCtlDel;

    let listener = TcpListener::bind("127.0.0.1:10001").unwrap();

    let epfd = epoll_create1(EpollCreateFlags::empty()).unwrap();

    let listen_fd = listener.as_raw_fd();
    let mut ev = EpollEvent::new(epoll_in, listen_fd as u64);
    epoll_ctl(epfd, epoll_add, listen_fd, &mut ev).unwrap();

    let mut fd2buf = HashMap::new();
    let mut events = vec![EpollEvent::empty(); 1024];

/*
 * prifri, 2022.12.08:
 * - epoll_wait
 *   두번째 인수에 전달된 슬라이스에 발생한 event의 개수를 return한다.
 *   3번째 인수의 -1은 block timeout을 의미.
 * - return은 발생한 이벤트 개수가 return
 */
    while let Ok(nfds) = epoll_wait(epfd, &mut events, -1) {
        for n in 0..nfds {
/*
 * prifri, 2022.12.08:
 * - tcp라고 하면 보통 bind -> listen -> send / recv의 시나리오를 해서
 *   listen api도 따로 있고 하는 구조인데 여기선 epoll을 사용해
 *   listen + send /recv를 한번에 wait하게 한게 보인다.
 * - listen에서 이벤트가 발생했으면 listen발생시의 tcp처리를 수행한다.
 *   listen으로 들어온 section에 대해 fd를 만들고 해당 fd를 위한 reader, writer
 *   를 만들어 fd로 HashMap에 등록하고, 설정이 완료된 fd를 epoll에 추가하여
 *   다음부터 read수행이 되기를 기다린다.
 */
            if events[n].data() == listen_fd as u64 {
                if let Ok((stream, _)) = listener.accept() {
                    let fd = stream.as_raw_fd();
                    let stream0 = stream.try_clone().unwrap();
                    let reader = BufReader::new(stream0);
                    let writer = BufWriter::new(stream);

                    fd2buf.insert(fd, (reader, writer));
                    println!("accept: fd = {}", fd);

                    let mut ev =
                        EpollEvent::new(epoll_in, fd as u64);
                    epoll_ctl(epfd, epoll_add, fd, &mut ev).unwrap();
                }
            } else {
/*
 * prifri, 2022.12.08:
 * - recv event를 감지했다. 아직 read는 안한 상태.
 */
                let fd = events[n].data() as RawFd;
/*
 * prifri, 2022.12.08:
 * - hashmap에 넣어놨던 fd의 read, write를 가져온다.
 */
                let (reader, writer) =
                    fd2buf.get_mut(&fd).unwrap();

                let mut buf = String::new();
/*
 * prifri, 2022.12.08:
 * - 실제 read를 수행한다.
 */
                let n = reader.read_line(&mut buf).unwrap();

                if n == 0 {
/*
 * prifri, 2022.12.08:
 * - n == 0이면 recv가 아닌 error로 인한 event로 판단한다.
 *   사실 network처리가 그렇듯 epoll의 error처리도 case에 따라 상당히 복잡하지만
 *   여기선 그냥 러프하게 close.
 */
                    let mut ev =
                        EpollEvent::new(epoll_in, fd as u64);
                    epoll_ctl(epfd, epoll_del, fd, &mut ev).unwrap();
                    fd2buf.remove(&fd);
                    println!("closed: fd = {}", fd);
                    continue;
                }

                print!("read: fd = {}, buf = {}", fd, buf);
/*
 * prifri, 2022.12.08:
 * - read한만큼 wrtie
 */
                writer.write(buf.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
        }
    }
}
