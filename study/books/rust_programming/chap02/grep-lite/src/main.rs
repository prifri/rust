use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
/*
 * prifri, 2022.09.21:
 * - regex 크레이트의 regex 타입을 지역범위로 가져온다.
 */
use regex::Regex;

/*
 * prifri, 2022.09.21:
 * - clap::App, clap::Arg 객체를 지역 범위로 가져온다.
 */
use clap::{App, Arg};

fn test() {

/*
 * prifri, 2022.09.21:
 * - unwrap()은 result값을 풀어내는데 오류가 발생하면 강제 종료한다.
 */
    let re = Regex::new("picture").unwrap();
    //let search_term = "picture";

    let quote = "Every face, every shop, bedroom window, public-house, and\n\
    dark square is a picture feverishly turned--in search of what?\n\
    It is the same with books, What do we seek through millions of pages?";

    /*
    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
    */

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {

/*
 * prifri, 2022.09.21:
 * - Some(T)
 * Option type의 값 중 긍정ㅈ억인 경우로, re.find()가 성공했다는 의미.
 * 결과가 있는 모든 경우에 해당한다.
 * - None
 * Option type의 값 중 부정적인 경우. ()는 널자리 표시자로 볼수있다.
 */
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn test2() {

/*
 * prifri, 2022.09.21:
 * - 명령 인자 분석기를 점진적으로 구성한다. Arg를 통해 각 인자를 가져온다.
 */
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
             .help("The pattern to search for")
             .takes_value(true)
             .required(true))
        .get_matches();


    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and\n\
    dark square is a picture feverishly turned--in search of what?\n\
    It is the same with books, What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn test3() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
             .help("The pattern to search for")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("input")
             .help("File to search")
             .takes_value(true)
             .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();

/*
 * prifri, 2022.09.21:
 * -line은 String type. re.find()는 &str을 인자로 받는다고한다. 
 */
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn test4() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
             .help("The pattern to search for")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("input")
             .help("File to search")
             .takes_value(true)
             .required(false))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}

fn main() {
    //test();
    //test2();
    //test3();
    test4();
}
