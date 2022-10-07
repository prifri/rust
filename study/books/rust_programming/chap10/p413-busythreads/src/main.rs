use std::{thread, time, hint};

/*
 * prifri, 2022.10.08:
 * - while문을 사용해서 pop을 하는 이유
 *   join을 해야된다 -> handle을 수정해야된다 -> for문에서는 read only 참조로
 *   가져 올 경우 이게 안된다 -> pop으로 직접 뽑아오면서 write도 
 *   가능한 상태로 한다.
 *   결국 writeable만 되면 되기 때문에 
 * - Some을 쓰는 이유.
 *   pop에서 뽑았으면 Some을써야되나보다 .ㅎㅎ 잘모른다.
 */
fn test1(mut handlers: Vec<thread::JoinHandle<()>>)
{
        while let Some(handle) = handlers.pop() {
            match handle.join() {
                Ok(_) => (),
                Err(_) => (),
            }
        }
}


/*
 * IAMROOT, 2022.10.08:
 * - 불가능한 방법. 참조로 가져오면 for문에선 readonly만 된다.
 */
/*
fn test2(mut handlers: Vec<thread::JoinHandle<()>>)
{
        for handle in &handlers {
            match handle.join() {
                Ok(_) => (),
                Err(_) => (),
            }
        }
}
*/


/*
 * IAMROOT, 2022.10.08:
 * - 유효한 방법. writeable이 가능하다.
 */
fn test3(mut handlers: Vec<thread::JoinHandle<()>>)
{
        for handle in handlers {
            match handle.join() {
                Ok(_) => (),
                Err(_) => (),
            }
        }
}

fn main() {
    for n in 1..=1000 {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

        let start = time::Instant::now();
        for _mn in 0..=n {
            let handle = thread::spawn(|| {
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20);

                while start.elapsed() < pause {

/*
 * IAMROOT, 2022.10.08:
 * - 잠깐 쉰다. 잠깐의 개념이 현재 thread에 주어진 timeslice를 포기한다는
 * 것이다. 이런 방식을 busy wait strategy라고 부른다고 하며,
 * 반복적인 polling이나 event가 발생했을때 쓸수있다고 한다.
 */
                    thread::yield_now();
/*
 * prifri, 2022.10.08:
 * - std::sync::atomic::spin_loop_hint();
 *   spin loop에 있다는걸 cpu에 알린다. hint::spin_loop로 대체되었다.
 */
                    hint::spin_loop();
                }

            });
            handlers.push(handle);
        }

        test1(handlers);

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}
