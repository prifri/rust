use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(10);
    {
/*
 * prifri, 2022.12.01:
 * - 10에대한 read lock을 얻는다.
 */
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }

    {
/*
 * prifri, 2022.12.01:
 * - 10에대한 write lock을 얻는다.
 */
        let mut v = lock.write().unwrap();
        *v = 7;
        println!("v = {}", v);
    }
    {
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }
}
