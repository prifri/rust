//*******************************************
//fence미사용
//*******************************************
#![allow(unused_variables)]
//*******************************************

use std::ptr::{read_volatile, write_volatile};
use std::thread;

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 5000;

/*
 * prifri, 2022.12.02:
 * - volatile
 *   compiler 최적화를 방지
 * - 매크로는 ordering 최적화를 안한다고 한다.
 *   매크로 자체가 ordering 최적화를 안할거 같진않는데 오역이
 *   뭔지 책에 일단 그렇게 써있긴하다.
 *   macro미사용 보다는 volatile 미사용이라고 하는게 더
 *   좋은 표현일듯 싶다.
 */

//*******************************************
// macro
//*******************************************

//***************
// 사용
// _mem( -> _mem!(  으로 고쳐야된다.
//***************
/*
macro_rules! read_mem {
    ($addr: expr) => { unsafe { read_volatile($addr) } };
}

macro_rules! write_mem {
    ($addr: expr, $val: expr) => {
    unsafe { write_volatile($addr, $val) };
    };
}
*/

//***************
// 미사용
//***************

fn read_mem<T>(addr : *const T) -> T {
unsafe { read_volatile(addr) }
}

fn write_mem<T>(addr: *mut T, val: T) {
unsafe { write_volatile(addr, val) };
}

//*****************
// macro + volatile 미사용.
//*****************

/*
fn read_mem<T>(addr : *const T) -> T where T : Copy {
    unsafe {*addr}
}

fn write_mem<T>(addr: *mut T, val: T) {
    unsafe { *addr = val };
}
*/

/*
 * prifri, 2022.12.02:
 * - fence, volatile 둘중 하나라도 안쓰면 프로그램이 정상적으로 동작하지 않을수 있다.
 * - fence를 안쓰는 경우
 *   ticket get을 위한 entering의 false / true write가 바뀔수있다.
 *   함수에 하나의 code에서 entering을 false -> (무언가함) -> true 하는데
 *   cpu가 볼때는 (무언가)를 하고 entering에 true를 하는것보다
 *   entering근처 address를 한번에 접근해서 한번에 그냥 해버리는게 좋기때문에
 *   entering을 false -> true -> (무언가함) 이런식으로 해버릴수있다.
 *   이렇게되면 ticket get lock이 동작을 안하므로 중복된 ticket을 가져올수있고,
 *   ticket을 가지고온 상태에서 기다리다가 차례가 오면 같은 ticket을 가지고
 *   있는 thread가 동시에 풀려 COUNT를 동시에 접근하는 경우가 생긴다.
 *   이렇게 되면 최종적으로 COUNT는 NUM_LOOP * NUM_THREADS에 못미치게된다.
 *   x86이나 amd에서는 ordering 최적화를 잘안해줘서 fence를 안써도 해당
 *   결과가 잘안나오지만 arm64에서는 책에서 말한것처럼 ordering최적화를
 *   적극적으로 하므로 거의 100% 현상이 발생한다.
 *
 * - volatile을 안쓰는 경우
 *   volatile을 안쓰면 compile가 code자체를 최적화를 통해 없애버릴수있다.
 *   code의 구조가 entring의 관점에서
 *
 *   entering의 false -> (..) -> true -> (..) -> while read entering -> ..
 *
 *   이렇게 되는데, compile러가 봤을때는 시나리오만 봐서는 마치 
 *
 *  1)
 *   entering의 false -> (..) -> true -> (..) -> while read entering -> ..
 *                               ^
 *                               어 여기서 true가 되네? 그냥 entering은 true
 *
 *  2)
 *   true -> (..) -> while (entering) {} -> ..
 *                       ^
 *                      어 entering은 언제나 true네? 그냥 무한 while (1) {}
 *  3)
 *   true -> (..) -> while (1) {} -> ..
 *                      ^ 무한 loop걸려서 code 동작안함.
 *
 *   이렇게 되서 code가 무한 loop에 빠지고 정상적으로 동작을 안하게 된다.
 */
//use std::sync::atomic::{fence, Ordering};
//*******************************************
//fence미사용
//*******************************************
use std::sync::atomic::Ordering;
fn fence(order: Ordering) {
}
//*******************************************

struct BakeryLock {
    entering: [bool; NUM_THREADS],
/*
 * prifri, 2022.12.02:
 * - lock을 안가지고 있다는 의미로 None을 넣기 위해 Option을 썻다.
 */
    tickets: [Option<u64>; NUM_THREADS]
}

impl BakeryLock {
    fn lock(&mut self, idx: usize) -> LockGuard {

        /*
         * prifri, 2022.12.02:
         * - entering 으로 lock(ticket get)중이라는것을 표시
         */
        fence(Ordering::SeqCst);
        write_mem(&mut self.entering[idx], true);
        fence(Ordering::SeqCst);

        let mut max = 0;
        for i in 0..NUM_THREADS {
            if let Some(t) = read_mem(&self.tickets[i]) {
                max = max.max(t);
            }
        }

        /*
         * prifri, 2022.12.02:
         * - 최대값 + 1을 자신의 ticket으로 한다.
         */
        let ticket = max + 1;
        write_mem(&mut self.tickets[idx], Some(ticket));

        fence(Ordering::SeqCst);
        /*
         * prifri, 2022.12.02:
         * - lock(ticket get)이 끝낫으므로 entering해제
         */
        write_mem(&mut self.entering[idx], false);
        fence(Ordering::SeqCst);

        /*
         * prifri, 2022.12.02:
         * - 모든 thread가 현재 자기보다 낮은 우선순위를 가지고 있는지 확인한다.
         * 1. 자신이 가진 ticket기 가장 낮은 번호.
         * 2. ticket번호가 동일하다면 thread number가 작다면.
         *
         * - 자신이 가장 높은 우선순위를 가지게되면 lock을 얻고 idx를 return return.
         */
        for i in 0..NUM_THREADS {
            /*
             * prifri, 2022.12.02:
             * - 자기자신 제외
             */
            if i == idx {
                continue;
            }
            /*
             * prifri, 2022.12.02:
             * - 해당 thread lock중(바로 직전 code)이면 풀릴때까지 대기.
             */
            while read_mem(&self.entering[i]) {}

            loop {
                match read_mem(&self.tickets[i]) {
                    /*
                     * prifri, 2022.12.02:
                     * - ticket 번호가 작거나, 같아도 자기보다 번호가 낮은경우 break.
                     */
                    Some(t) => {
                        if ticket < t ||
                            (ticket == t && idx < i) {
                                break;
                            }
                    }
                    /*
                     * prifri, 2022.12.02:
                     * - ticket이 없다면 break. 대기중이 아무것도없다는뜻.
                     */
                    None => {
                        break;
                    }
                }
            }
        }

        fence(Ordering::SeqCst);
        LockGuard { idx }
    }
}


struct LockGuard {
    idx: usize
}

/*
 * prifri, 2022.12.02:
 * - Drop::drop trait.
 *   unlock될시 call된다. unlock될때 tickets를 초기화 해야되므로 그 처리를
 *   수행한다.
 */
impl Drop for LockGuard {
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        /*
         * prifri, 2022.12.02:
         * - rust에서는 mutable global 변수 이용을 권장하지 않고, 접근은 모두 unsafe인데,
         *   예제를 위하 그냥 사용했다고 한다.
         */
        unsafe {
            write_mem(&mut LOCK.tickets[self.idx], None);
        };
    }
}

static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};

static mut COUNT: usize = 0;

fn monitor() {
    unsafe {
        let mut before_count = 999999;
        while before_count != COUNT {
            for i in 0..NUM_THREADS {
                print!(" {} ", LOCK.entering[i]);
                if let Some(t) = LOCK.tickets[i] {
                    print!("{} ||", t);
                } else {
                    print!("X ||");
                };
            }
            println!(" {}", COUNT);
            before_count = COUNT;
            thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}

fn main() {
    let mut v = Vec::new();
    for i in 0..NUM_THREADS {
        let th = thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let _lock = unsafe { LOCK.lock(i) };
                /*
                 * prifri, 2022.12.02:
                 * - lock을 얻은 채로 COUNT++을 진행한다. 이것을 진행와는 와중에도 다른
                 *   thread가 lock을 얻고 있을것이다.
                 */
                unsafe {
                    let c = read_volatile(&COUNT);
                    write_volatile(&mut COUNT, c + 1);
                }
            }
        });
        v.push(th);
    }

    let th = thread::spawn(move || { monitor() });
    v.push(th);

    for th in v {
        th.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})", unsafe { COUNT },
        NUM_LOOP * NUM_THREADS);
}
