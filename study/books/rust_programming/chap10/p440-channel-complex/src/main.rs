#[macro_use]
extern crate crossbeam;

use crossbeam::channel::unbounded;
use std::thread;

use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

fn main() {
    let n_messages = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (response_tx, response_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            Pong => eprintln!("unexpected pong response"),
            Ping => response_tx.send(Pong).unwrap(),
            Pang => return,
        }
    });

    for _ in 0..n_messages {
        requests_tx.send(Ping).unwrap();
    }

    requests_tx.send(Pang).unwrap();

/*
 * prifri, 2022.10.11:
 * - 생성된 thread와 다르게 requests로 msg 4번보내고 select!에서 wait한다.
 */
    for _ in 0..n_messages {
        select! {
            recv(response_rx) -> msg => eprintln!("{:?}", msg),
        }
    }
}
