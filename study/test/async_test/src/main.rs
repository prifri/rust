use futures::executor::block_on;

async fn foo() {
    println!("foo");
}

async fn bar() {
    println!("poo");
}

async fn foobar() {
    foo().await;
    bar().await;
}

fn main() {
    block_on(foobar());
}
