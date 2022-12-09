use futures::executor::block_on;
use std::future::Future;

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}
//********************************************************************
// test1 https://www.youtube.com/watch?v=ThjvMReOXYM&t=437s
//********************************************************************
/*
 * prifri, 2022.12.09:
 * - fn앞의 async keyword는 foo1_2함수와 같은 의미한다.
 */
async fn foo1() {
    println!("{} {}", function!(), line!());
}

fn foo1_2() -> impl Future<Output = ()> {
    async {
        println!("{} {}", function!(), line!());
    }
}

fn test1() {
    block_on(foo1());
    block_on(foo1_2());
}

//********************************************************************
// test2 https://www.youtube.com/watch?v=ThjvMReOXYM&t=437s
//********************************************************************
/*
 * prifri, 2022.12.09:
 * - return형은 다음과 같은 의미로 동치다.
 */
async fn foo2() -> usize {
    println!("{} {}", function!(), line!());
    0
}

fn foo2_2() -> impl Future<Output = usize> {
    async {
        println!("{} {}", function!(), line!());
        0
    }
}

fn test2() {
    block_on(foo2());
    block_on(foo2_2());
}

//********************************************************************
// test3 https://www.youtube.com/watch?v=ThjvMReOXYM&t=437s
//********************************************************************

async fn foo3() -> usize {
    println!("{} {}", function!(), line!());
    0
}

fn test3() {
/*
 * prifri, 2022.12.09:
 * - compile error. return형은 Future<Output = usize>이기 때문이다.
 */
    //let x: usize = foo3();
/*
 * prifri, 2022.12.09:
 * - 안에 내용이 출력되지 않을것이다. 위 함수의미 자체가 정의 일 뿐이지
 *   실행이 아니기 때문이다. 마치 callback = foo3만 하고
 *   callback()를 실행안한것과 같다.
 */
    let _  = foo3();
}

//********************************************************************
// test4 https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
//********************************************************************

async fn foo4() {
    println!("{} {}", function!(), line!());
}

async fn bar4() {
    println!("{} {}", function!(), line!());
}

async fn foobar4() {
    foo4().await;
    bar4().await;
}

fn test4() {
    block_on(foobar4());
}

//********************************************************************
// test5 https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
//********************************************************************

struct Song {
    a: i32,
}

async fn learn_song() -> Song {
    for i in 0..1 {
        println!("{} {} {}", function!(), line!(), i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("{} {}", function!(), line!());
    Song {a: 1 }
}

async fn sing_song(song: Song) {
    let _ = song.a;
    for i in 0..1 {
        println!("{} {} {}", function!(), line!(), i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("{} {}", function!(), line!());
}

async fn dance() {
    println!("{} {}", function!(), line!());
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
/*
 * prifri, 2022.12.09:
 * - rust 공식 예제인데 뭔가 예를 잘못들었다고해야되나 대충 설명해줬다.
 *   웃긴게 마치 await를하면 두개가 동시에 실행되는것처럼 얘기하지만
 *   실젠 learn_song(); 호출과 다를바없다. 즉 이런방식으로 써서는 의미가없다.
 * - rust공식문서에는 이런 설명이 종종있을뿐만 아니라 아에 실습 자체가
 *   하기 힘들정도로 지들끼리만 아는 내용만 말하는게 있다.
 *   초보자에게 도움이 하나도 안된다.
 */
    println!("{} {}", function!(), line!());
    let song = learn_song().await;
    sing_song(song).await;
    println!("{} {}", function!(), line!());
}

async fn test5() {
    println!("{} {}", function!(), line!());
    let f1 = learn_and_sing();
    let f2 = dance();

    println!("{} {}", function!(), line!());
    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
/*
 * prifri, 2022.12.09:
 * - f1, f2 동시에 block하는 개념
 */
    futures::join!(f1, f2);
    println!("{} {}", function!(), line!());
}

//********************************************************************
// test6 https://www.youtube.com/watch?v=K8LNPYNvT-U
//********************************************************************

async fn read_from_database() -> String {
    "DB Result".to_owned()
}

async fn my_function(i: i32) {
    println!("[{i}] {} {}", function!(), line!());
    let s1: String = read_from_database().await;
    println!("[{i}] {} {} {}", function!(), line!(), s1);
    let s2: String = read_from_database().await;
    println!("[{i}] {} {} {}", function!(), line!(), s2);
}

async fn test6() {
    let mut handles: Vec<tokio::task::JoinHandle<()>> = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
       tokio::time:: sleep(std::time::Duration::from_millis(50)).await;
        handle.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    test1();
    test2();
    test3();
    test4();
    block_on(test5());
    block_on(test6());
}
