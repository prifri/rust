use std::io::Read;

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = std::fs::File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_username_from_file2() -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, std::io::Error> {
    let mut s = String::new();
    std::fs::File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    //let vec = vec![1, 2, 3];
    //vec[99];

    let f = std::fs::File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match std::fs::File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("create rror");
                }
            }
        },
        Err(error) => {
            panic!("error");
        }
    };

    let f = read_username_from_file();
    let f = match f{
        Ok(s) => println!("user name : {} {}", s, s.len()),
        Err(e) => {
            println!("not found user name");
        }
    };
    let f = read_username_from_file2();
    let f = match f{
        Ok(s) => println!("user name : {} {}", s, s.len()),
        Err(e) => {
            println!("not found user name");
        }
    };
    let f = read_username_from_file3();
    let f = match f{
        Ok(s) => println!("user name : {} {}", s, s.len()),
        Err(e) => {
            println!("not found user name");
        }
    };
}
