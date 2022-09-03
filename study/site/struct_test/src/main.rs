struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn print_user(user : &User) {
    println!("{} {} {} {}",
             user.email,
             user.username,
             user.active,
             user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn square(size : u32) -> Rectangle {
        Rectangle { length: size, width: size}
    }
}

fn main() {
    let user1 = User {
        email: String::from("abc@eamp.com"),
        username: String::from("aaaa"),
        active: true,
        sign_in_count: 1,
    };

    //error
    //user1.active = false;
    print_user(&user1);

    let mut user1 = User {
        email: String::from("abc@eamp.com"),
        username: String::from("aaaa"),
        active: true,
        sign_in_count: 1,
    };

    user1.active = false;
    print_user(&user1);

    let user2 = build_user(String::from("abc@abc.com"), String::from("abc"));
    print_user(&user2);

    let user2 = User {
        email: String::from("aaa@aaa.com"),
        ..user1
    };
    print_user(&user2);

    //struct Color(i32, i32, i32);
    //struct Point(i32, i32, i32);

    //let c1 = Color(1, 2, 3);
    //let p1 = Point(4, 5, 6);

    let r1 = Rectangle { length: 10, width: 20 };

    println!("r1 {}", r1.area());

    let r2 = Rectangle::square(10);

    println!("r2 {}", r2.area());
    //println!("debug is {:?}", r2);
}
