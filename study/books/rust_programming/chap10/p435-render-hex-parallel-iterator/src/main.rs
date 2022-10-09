use std::env;

use svg::Document;
use svg::node::element::{Path, Rectangle};
use svg::node::element::path::{Command, Position, Data};

use rayon::prelude::*;


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
 * IAMROOT, 2022.10.09:
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

fn parse(input: &str) -> Vec<Operation> {

/*
 * prifri, 2022.10.10:
 * - as_bytes()
 *   input(입력 문자열 슬라이스)를 바이트 슬라이스로 변환한다.
 * - par_iter()
 *   바이트 슬라이스를 병렬 반복자로 변환한다. 경쟁 조건이 생기지 않도록
 *   보장한다.
 */
    input
        .as_bytes()
        .par_iter()
        .map(|byte|{
        match byte {
            b'0'  => Home,
            b'1'..=b'9' => {
                let distance = (byte - 0x30) as isize;
                Forward(distance * (HEIGHT / 10))
            },
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,

            /*
            * IAMROOT, 2022.10.10:
            * - byte 변수는 &u8타입으로, Operation::Noop(u8)에 맞출려면
            *   값을 역참조해야한다.,
            */
            _ => Noop(*byte),
        }}).collect()
}

/*
 * IAMROOT, 2022.10.09:
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
 * IAMROOT, 2022.10.09:
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
 * IAMROOT, 2022.10.09:
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
 * IAMROOT, 2022.10.09:
 * - svg 생성 파이프라인
 */
    let operations = parse(input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap();
}
