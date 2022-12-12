use futures::executor::block_on;
use std::sync::{Arc, Mutex};

struct State {
    count: u64,
}

async fn task2(state: &Arc<Mutex<State>>) -> u64 {
    if let Ok(mut state) = state.lock() {
        state.count += 2;
    }
    println!("H 2");
    2
}

async fn task1(state: &Arc<Mutex<State>>) -> u64 {
    if let Ok(mut state) = state.lock() {
        state.count += 1;
    }
    println!("H 1");
    1
}

async fn async_main() {
    let state = State { count: 0 };
    let data = Arc::new(Mutex::new(state));
    //task1(&mut state).await;
    //task2(&mut state).await;
    let (res1, res2) = futures::join!(task1(&data), task2(&data));

    println!("Result #1: {} and Result #2: {}", res1, res2);

    if let Ok(s) = data.lock() {
        println!("State: {}", s.count);
    };
}

fn main() {
    block_on(async_main());
}
