struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> std::fmt::Display for MyBox<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = *y;
    let k1 = Box::new(y);
    let k2 = Box::new(*y);
    println!("{} {} {} {} {}", x, y, z, k1, k2);

    let m = MyBox::new(x);
    println!("{}", m);
    println!("{}", *m);
}
