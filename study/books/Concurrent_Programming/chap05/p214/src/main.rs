use std::sync::{Arc, Mutex};

const NUM_TASKS: usize = 2;
const NUM_LOOP: usize = 100000;

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    let val = Arc::new(Mutex::new(0));
    let mut v = Vec::new();
    for i in 0..NUM_TASKS {
        let n = val.clone();
        println!("{} create", i);
        let t = tokio::spawn(async move {
            println!("{} spawn", i);
            for _ in 0..NUM_LOOP {
                let mut n0 = n.lock().unwrap();
                *n0 += 1;
            }
/*
 * prifri, 2022.12.12:
 * - cpu 점유율이 task에 따라서 멀티로 동작하는게 확인된다.
 */
            for _ in 0..1000000000 {
               // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            println!("{} spawn end", i);
        });
        println!("{} create end", i);
        v.push(t);
    }

    println!("{}", line!());
/*
 * prifri, 2022.12.12:
 * - 여기서 비동기로 동작한다.
 */
    for i in v {
        i.await?;
    }
    println!("{}", line!());

    println!("COUNT = {} (expected = {})",
        *val.lock().unwrap(), NUM_LOOP * NUM_TASKS);
    Ok(())
}
