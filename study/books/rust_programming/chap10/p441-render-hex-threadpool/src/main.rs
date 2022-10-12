use std::thread;
use std::env;

use svg::Document;
use svg::node::element::{Path, Rectangle};
use svg::node::element::path::{Command, Position, Data};

use crossbeam::channel::unbounded;

/*
 * prifri, 2022.10.09:
 * - 뒤에 enum정의를 해놓고 여기서 use를 써서 prefix를 생략시킨다.
 */
use crate::Operation::{
    Forward,
    TurnLeft,
    TurnRight,
    Home,
    Noop
};

use crate::Orientation::{
    North,
    East,
    West,
    South
};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

const HOME_X: isize = HEIGHT/2;
const HOME_Y: isize = WIDTH/2;

/*
 * prifri, 2022.10.09:
 * - 그림의 그릴때의 매개변수.
 */
const STROKE_WIDTH: usize = 5;
#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    West,
    South
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    /*
    * prifri, 2022.10.09:
    * - 잘못된 값을 받았을시 Noop.
    */
    Noop(u8)
}

#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    fn new() -> Artist {
        Artist {
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            West => self.x += distance,
            East => self.x -= distance,
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }

/*
 * prifri, 2022.10.09:
 * - 그림이 경계안에 있는걸 확인.
 */
    fn wrap(&mut self) {
        if self.x < 0{
            self.x = HOME_X;
            self.heading = West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = East;
        }

        if self. y < 0 {
            self.y = HOME_Y;
            self.heading = North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = South;
        }
    }
}

/*
 * prifri, 2022.10.11:
 * - 채널을 통해 보낼 메시지 타입. usize는 current pos.
 */
enum Work {
    Task((usize, u8)),
    Finished,
}

fn parse_byte(byte: u8) -> Operation {
    match byte {
        b'0' => Home,
        b'1'..=b'9' => {
            let distance = (byte - 0x30) as isize;
            Forward(distance * (HEIGHT / 10))
        },
        b'a' | b'b' | b'c' => TurnLeft,
        b'd' | b'e' | b'f' => TurnRight,
        _ => Noop(byte),
    }
}

fn parse(input: &str) -> Vec<Operation> {
    let n_threads = 2;
    let (todo_tx, todo_rx) = unbounded();
    let (results_tx, results_rx) = unbounded();
    let mut n_bytes = 0;

/*
 * prifri, 2022.10.11:
 * - 데이터를 미리 다 보내놓는다.
 */
    for (i, byte) in input.bytes().enumerate() {
        todo_tx.send(Work::Task((i, byte))).unwrap();
        n_bytes += 1;
    }

/*
 * prifri, 2022.10.11:
 * - 완료까지 보내놓는다.
 */
    for _ in 0..n_threads {
        todo_tx.send(Work::Finished).unwrap();
    }

    for _ in 0..n_threads {
/*
 * prifri, 2022.10.11:
 * - 복제된다. thread간에 동기화는 채널에서 자동으로 이뤄지게 될것이다.
 */
        let todo = todo_rx.clone();
        let results = results_tx.clone();
        thread::spawn(move || {
            loop {
                let task = todo.recv();
                let result = match task {
                    Err(_) => break,
                    Ok(Work::Finished) => break,
                    Ok(Work::Task((i, byte))) => (i, parse_byte(byte)),
                };
                results.send(result).unwrap();
            }
        });
    }
    let mut ops = vec![Noop(0); n_bytes];

/*
 * prifri, 2022.10.11:
 * - 결과는 임의의 순서를 반환할수있으므로 이전에 보냇던 index에 완성된
 *   op를 넣는 식으로 진행한다.
 * - 배열을 사용안하는 이유는 타입시그니처 때문이라고 하며, 추후에 변경될때
 *   리팩토링을 하기 싫다는 이유라고 하며, 실제로 이런이유로 아주 특별하게
 *   최적화가 필요하지 않으면 배열대신 vector를 그냥 사용한다.
 */
    for _ in 0..n_bytes {
        let (i, op) = results_rx.recv().unwrap();
        ops[i] = op;
    }
    ops
}

/*
 * prifri, 2022.10.09:
 * - Operation -> Command 변환.
 */
fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut turtle = Artist::new();

    let mut path_data = Vec::<Command>::with_capacity(operations.len());
    let start_at_home = Command::Move(
        Position::Absolute, (HOME_X, HOME_Y).into()
        );
    path_data.push(start_at_home);

    for op in operations {

/*
 * prifri, 2022.10.09:
 * - op로 command 를 만들고, x, y값을 line(그리는데 사용)을 생성한다음
 *   path에 만든 line을 먼들어놓는다.
 */
        match *op {
            Forward(distance) => turtle.forward(distance),
            TurnLeft => turtle.turn_left(),
            TurnRight => turtle.turn_right(),
            Home => turtle.home(),
            Noop(byte) => {
                eprintln!("warning: illegal byte encountered: {:?}", byte);
            },
        };

        let path_segment = Command::Line(
            Position::Absolute, (turtle.x, turtle.y).into()
            );
        path_data.push(path_segment);

/*
 * prifri, 2022.10.09:
 * - 경계를 넘은지 확인. 중앙 복귀.
 */
        turtle.wrap();
    }
    path_data
}

fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "None")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    let document = Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border);

    document
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    let default_filename = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default_filename);

/*
 * prifri, 2022.10.09:
 * - svg 생성 파이프라인
 */
    let operations = parse(input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap();
}
