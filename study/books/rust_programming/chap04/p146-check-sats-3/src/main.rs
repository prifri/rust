#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?} {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

/*
 * prifri, 2022.10.03:
 * - sat_a의 소유권이
 * main(variable binding)  -> check_status -> main의 경로로 복귀한다.
 */
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    // 대기 중
    
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
}
