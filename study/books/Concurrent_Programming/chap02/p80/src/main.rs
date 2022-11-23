use std::thread::spawn;

fn hello() {
    println!("Hello World!");
}

fn p80() {
    spawn(hello).join().unwrap();

    let h = || println!("heelo world!");
    spawn(h).join().unwrap();
}

fn p81() {
    let v = 10;
    let f = move || v * 2;

/*
 * prifri, 2022.11.23:
 * - join의 반환은 Result type이다. 이 경우 Ok(20)을 포함해서 반환한다.
 */
    let result = spawn(f).join();
    println!("result = {:?}", result);

/*
 * prifri, 2022.11.23:
 * - panic하는 thread를 생성한다.
 * - 올바르게 종료되면 위처럼 join에 Ok로 반환된다.
 * - panic으로 종료된 경우 join 함수의 반환값에 Result 타입의 Error에 패닉시의
 * 값이 포함된다. Err에 포함된 값의 타입은 어떤 타입도 될수 있는 Any라 불리는
 * 특수 타입이다(void 같은거인듯). 이 Any type에 아마 &str로 에러 string값이
 * 오도록 암묵적 약속이 있나보다. &str로 변환해 println에 표시한다.
 */
    match spawn(|| panic!("I'm panicked!")).join() {
        Ok(_) => {
            println!("successed");
        }
        Err(a) => {
            let s = a.downcast_ref::<&str>();
            println!("failed: {:?}", s);
        }
    }

    println!("end");
}

fn main() {
    p80();
    p81();
}
