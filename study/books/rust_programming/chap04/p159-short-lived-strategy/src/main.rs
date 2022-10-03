#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

struct GroundStation {}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to != recipient.id {
                continue;
            }

            let msg = self.messages.remove(i);

            return Some(msg);
        }

        None
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id
        }
    }

    fn send(&self, mailbox:&mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };

    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();

/*
 * prifri, 2022.10.03:
 * - for문안에서 msg를 만들고 mailbox로 소유권을 옮긴다.
 */
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message { to: sat_id, content: String::from("hello") };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();

/*
 * prifri, 2022.10.03:
 * - mailbox에서 msg를 꺼내서 출력한다.
 */
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}
