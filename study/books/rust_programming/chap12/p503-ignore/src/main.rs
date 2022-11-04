use libc::{signal, raise};
use libc::{SIG_DFL, SIG_IGN, SIGTERM};

fn main() {
/*
 * prifri, 2022.11.04:
 * - SIGTERM을 무시(IGNORE)한다. 만약 이 설정이 안먹혓으면 ok를 안찍고
 * 프로그램이 종료될것이다.
 */
    unsafe {
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }

    println!("ok");
/*
 * prifri, 2022.11.04:
 * - SIGTERM을 원상태로 복귀한다. nok를 못찍고 프로그램은 종료될것이다.*/
    unsafe {
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }

    println!("nok");

}
