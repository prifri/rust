
/*
 * prifri, 2022.09.26:
 * - 열거형은 Debug를 쓰면 화면에 출력가능.
 */
#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

fn parse_log(line: &'static str) -> (Event, Message) {

/*
 * prifri, 2022.09.26:
 * - collect는 line.splitn()에서 생성된 반복자를 써서 Vec<T>를 반환한다.
 * - splitn(2, ' ') log를 ' '를 기준으로 2개(0번, 1번)으로 나눈다.
 */
    let parts: Vec<&str> =
        line.splitn(2, ' ')
        .collect();

/*
 * prifri, 2022.09.26:
 * - 2개로 못나눴으면 error.
 */
    if parts.len() == 1 {
        return (Event::Unknown, String::from(line))
    }

/*
 * prifri, 2022.09.26:
 * - 0번이 event, 1번이 나머지 string.
 */
    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}

fn main() {
    let log = "BEGIN Transaction XK342\n\
    UPDATE 234:LS/32232 {\"price\": 32.00} -> {\"price\": 40.00}\n\
    DELETE 342:L0/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
