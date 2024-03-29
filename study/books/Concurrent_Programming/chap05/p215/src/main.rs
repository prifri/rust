use std::{sync::Arc, time};
use tokio::sync::Mutex;

const NUM_TASKS: usize = 0;

async fn lock_only(v: Arc<Mutex<u64>>) {
    let mut n = v.lock().await;
    *n += 1;
}

async fn lock_sleep(v: Arc<Mutex<u64>>) {
/*
 * prifri, 2022.12.12:
 * - lock을 얻은 상태에서 await를 수행.
 */
    let mut n = v.lock().await;
    let ten_secs = time::Duration::from_secs(10);
    tokio::time::sleep(ten_secs).await;
    *n += 1;
}

/*
 * prifri, 2022.12.12:
 * - lock을 획득한 상태에서 await를 수행.
 */
#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    let val = Arc::new(Mutex::new(0));
    let mut v = Vec::new();

/*
 * prifri, 2022.12.12:
 * - lock을 얻고 sleep하는 task spawn.
 */
    let t = tokio::spawn(lock_sleep(val.clone()));
    v.push(t);

/*
 * prifri, 2022.12.12:
 * - lock을 얻는 task spawn
 */
    for _ in 0..NUM_TASKS {
        let n = val.clone();
        let t = tokio::spawn(lock_only(n));
        v.push(t);
    }

/*
 * prifri, 2022.12.12:
 * - 실행
 */
    for i in v {
        i.await?;
    }
    Ok(())
}
