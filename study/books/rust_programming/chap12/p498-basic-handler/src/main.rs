/*
 * prifri, 2022.11.04:
 * - windows에서 사용안한다.
 */
#![cfg(not(windows))]

use std::time;
use std::thread::sleep;
use libc::{SIGTERM, SIGUSR1};

static mut SHUT_DOWN: bool = false;

fn register_signal_handlers() {
/*
 * prifri, 2022.11.04:
 * - libc호출은 안전하지 않다.
 * - 함수포인터형식으로 던진다. usize로 변환해서 한다. type정보를 감추고
 * 적당히 변환하는 기법인듯
 */
    unsafe {
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
    }
}

#[allow(dead_code)]
fn handle_sigterm(_signal: i32) {
/*
 * prifri, 2022.11.04:
 * - 이걸 여기서 왜 또 등록하는지 이해가 안간다.
 * 시그널 핸들러 자체에 영향을 미치는 시그널 변경을 최소화하기 위해 가능한한
 * 빨리 재등록 한다고 하는데 이해가 안간다.
 */
    register_signal_handlers();

    println!("SIGTERM");

    unsafe {
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)]
fn handle_sigusr1(_signal: i32) {
    register_signal_handlers();
    println!("SIGUSR1");
}
        
fn main() {
/*
 * prifri, 2022.11.04:
 * - signal 등록
 */
    register_signal_handlers();

    let delay = time::Duration::from_secs(1);

    for i in 1_usize.. {
        println!("{}", i);
        unsafe {
            if SHUT_DOWN {
                println!("*");
                return;
            }
        }

        sleep(delay);

        let signal = if i > 2 {
            SIGTERM
        } else {
            SIGUSR1
        };
/*
 * prifri, 2022.11.04:
 * - libc 호출은 안전하지 않다.
 */
        unsafe {
            libc::raise(signal);
        }
    }
    unreachable!();
}
