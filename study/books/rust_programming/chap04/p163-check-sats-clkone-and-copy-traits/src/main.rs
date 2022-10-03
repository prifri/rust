#[allow(dead_code)]
/*
 * prifri, 2022.10.03:
 * - Debug, Clone, Copy trait를 추가하라고 compiler한테 알린다.
 */
#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id: u64,
}

#[derive(Debug, Clone, Copy)]
enum StatusMessage {
    Ok,
}

#[allow(unused_variables)]
fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a.clone());
    println!("a: {:?}", a_status.clone());

/*
 * prifri, 2022.10.03:
 * - sat_a를 위에서 clone을 썻으므로 여기서 사용할수있다.
 * - 이렇게 되더라고 copy가 발생
 */
    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);

/*
 * prifri, 2022.10.03:
 * - copy가 되엇으니 역시 재사용 가능.
 */
    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}
