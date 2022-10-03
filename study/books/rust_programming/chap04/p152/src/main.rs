type Message = String;

#[derive(Debug)]
struct MailBox {
    message: Vec<Message>
}

#[derive(Debug)]
#[allow(dead_code)]
struct CubSat {
    id: u64,
    malibox: MailBox,
}

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubSat, msg: Message) {
        to.malibox.message.push(msg);
    }
}

impl CubSat {
    fn recv(&mut self) -> Option<Message> {
        self.malibox.message.pop()
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubSat {
        id: 0,
        malibox: MailBox {
            message: vec![]
        }
    };


/*
 * prifri, 2022.10.03:
 * - sat_a에 아무것도 없다.
 */
    println!("t0: {:?}", sat_a);

/*
 * prifri, 2022.10.03:
 * - sat_a에 넣었다.
 */
    base.send(&mut sat_a,
              Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

/*
 * prifri, 2022.10.03:
 * - sat_a에서 뺏다.
 */
    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}
