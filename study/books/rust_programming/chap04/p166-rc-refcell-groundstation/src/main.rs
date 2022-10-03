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
 * - borrow_mut
 *   가변참조자 ref up. scope를 벗어나면 자동으로 감소한다.
 *   만약 규칙을 어기고 ref up을 여러번 쓸경우 panic이 발생한다.
 * - 가변참조자를 늘린걸 base_2로 한다.
 */
        let mut base_2 = base.borrow_mut();

/*
 * prifri, 2022.10.03:
 * - panic 발생. 가변참조자를 한 scope에서 2번증가시켯기 때문이다.
 */
        //let mut base_2 = base.borrow_mut();

/*
 * prifri, 2022.10.03:
 * - 가변참조자를 늘린 상태이므로 변경 가능하다.
 */
        base_2.radio_freq -= 12.34;
        println!("base: {:?}", base);
        println!("base_2: {:?}", base_2);
    }

/*
 * prifri, 2022.10.03:
 * - 위에서 변경된값이 적용된게 확인된다.
 *   base_2로 가변참조자 ref를 up한 상태에서 변경이 됬고, scope가 닫히면서
 *   내부에서만 쓰던 base_2는 이제 안쓰고 원래 base의 값이 변경된게 확인.
 */
    println!("base: {:?}", base);

/*
 * prifri, 2022.10.03:
 * - 여기서도 가변 참조자를 ref up하고, ref up된 base는 borrowed가 되고,
 * base_3으로 변경이 된다.
 */
    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
