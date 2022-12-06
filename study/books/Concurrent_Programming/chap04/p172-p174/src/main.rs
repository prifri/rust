/*
 * prifri, 2022.12.06:
 * - rust의 차용 룰을 파기하고 뮤터블한 참조를 여럿 가지도록 하는 기술을
 * 가능하게 하는 위험한 타입이라고 한다.
 */
use std::cell::UnsafeCell;
/*
 * prifri, 2022.12.06:
 * - *를 이용해 참조 제외를 수행할수 있게 한다고 한다.
 * rust의 mutex는 lock했을 때 가드용 객체를 반환하는데,
 * 가드용 객체는참조 제외를 함으로써 보호 대상 데이터를 읽고 쓸수 있다고 한다.
 */
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;

struct SpinLock<T> {
    lock: AtomicBool,
    /*
     * prifri, 2022.12.06:
     * - 여러 thread가 mutable하게 접근할수 있으므로 unsafecell로 한다고 한다.
     */
    data: UnsafeCell<T>,
}

/*
 * prifri, 2022.12.06:
 * - 락 해제 및 락 안에 유지 대상 데이터의 참조를 취득하기 위한 타입.
 *   이 타입의 값이 스코프로부터 제외되었을 때 자동적으로 락이 해제되지만
 *   락을 해제하기 위해 SpinLock 타입의 참조를 유지한다고 한다.
 */
struct SpinLockGuard<'a, T> {
    spin_lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    fn new(v: T) -> Self {
        SpinLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(v),
        }
    }

/*
 * prifri, 2022.12.06:
 * - TTAS에 의해 락용 공용 변수가 false가 되어 락이 해제되는 것을 spin하며
 *   기다린다.
 */
    fn lock(&self) -> SpinLockGuard<T> {
        loop {
            while self.lock.load(Ordering::Relaxed) {}
/*
 * prifri, 2022.12.06:
 * - ..weak
 *   test에 성공하더라도 실패하면 재시도하지 않는다. weak가 없는 버전은
 *   재시도를 한다.
 */
            if let Ok(_) =
                self.lock
                    .compare_exchange_weak(
                        false,
                        true,
                        Ordering::Acquire,
                        Ordering::Relaxed)
                    {
                        break;
                    }
        }
        SpinLockGuard { spin_lock: self }
    }
}

/*
 * prifri, 2022.12.06:
 * - SpinLock 타입은 thread 사이에서 공유가능하도록 지정.
 *   rust의 mutex 타입등에서도 수행되고 있다고 한다. 일반적으로 이 타입에대한
 *   이런 지정은 수행할 필요가 없고 동기 메커니즘을 구현할때만 사용한다고 한다.
 */
unsafe impl<T> Sync for SpinLock<T> {}
/*
 * prifri, 2022.12.06:
 * - Send trait를 구현하면 채널을 통해 값을 송신할 수 있게 된다.
 */
unsafe impl<T> Send for SpinLock<T> {}

/*
 * prifri, 2022.12.06:
 * - SpinLockGuard가 scope에서 제외됬을때 호출된다.
 *   자동으로 락이 해제되도록 한다.
 */
impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.spin_lock.lock.store(false, Ordering::Release);
    }
}

/*
 * prifri, 2022.12.06:
 * - 참조 제외를 수행할 수있다. 여기에서는 보호 대상 데이터로의 참조를
 * 취득하도록 한다. 이렇게 함으로써 보호 시에 얻어진 SpinLockGuard 타입의
 * 값을 통해 보호 대상 데이터의 읽기쓰기가 가능해진다. 이와 같은 작업이
 * Rust의 mutexguard 타입에서도 수행된다.
 */
impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spin_lock.data.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spin_lock.data.get() }
    }
}

fn main() {
    let lock = Arc::new(SpinLock::new(0));
    let mut v = Vec::new();

    for _ in 0..NUM_THREADS {
        let lock0 = lock.clone();
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let mut data = lock0.lock();
                *data += 1;
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    println!("COUNT = {} (expected = {})", *lock.lock(), NUM_LOOP * NUM_THREADS);
}
