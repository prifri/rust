
/*
 * prifri, 2022.10.03:
 * - Rc<T>
 * T의 rc, T의 참조카운트값.
 * 공유 소유권을 제공한다. 모든 소유자가 삭제되기 전까지 T가 메모리에서삭제
 * 되는것을 막는다.
  - clone을 할대마다 참조 카운트가 1씩 증가한다.
  - runtime시 검사라 runtime시 비용이 있다.
  - readonly
    write able 할려면 RefCell을 써야한다.
 */
use std::rc::Rc;

#[derive(Debug)]
struct Groundstation {}

fn main() {
    let base = Rc::new(Groundstation {});

    println!("{:?}", base);
}
