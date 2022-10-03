use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64 // Mhz
}

/*
 * prifri, 2022.10.03:
 * - 참조카운트 기능 + 변동가능 기능까지 포함된다.
 * - Clone()이 상대적으로 비쌀 경우 쓸수있다.
 * - readonly인 Rc에 비해서 비용이 더 비싸다.
 * - Rc와 마찬가지로 runtime이다.
 * - Rc<T>, Rc<RefCell<T>>는 threadsafe가 아니다.
 *   Arc<T>, Arc<Mutex<T>>로 대체해야된다.
 *   arc는 원자적 참조 카운터(atomic reference counter).
 *   역시 비용은 훨신더 비싸다.
 */
fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
            GroundStation {
                radio_freq: 94.55
            }
            ));

    println!("base: {:?}", base);

    {
/*
 * prifri, 2022.10.03:
 * - base를 가변적으로 대여할 수 있는 새로운 범위를 도입
 */
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

/*
 * prifri, 2022.10.03:
 * - 위에서 변경된값이 적용된게 확인된다.
 */
    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
