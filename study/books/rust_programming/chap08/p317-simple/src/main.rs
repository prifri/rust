use std::error::Error;
use reqwest;

/*
 * PRIFRI, 2022.10.06:
 * - Box<dyn Error
 *   rust가 실행 시에 다형성을 지원할 수 있게 해 주는 trait 객체의 예.
 *   trait 객체는 구체적인 타입에 대한 프락시.
 *   Body<dyn std::error::Error> 구문은 std::error::Error을 구현하는 모든
 *   타입에 대한 Box(포인터)를 의미한다.
 */
fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
/*
 * PRIFRI, 2022.10.06:
 * - HTTP GET 요청
 */
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    print!("{}", content);

    Ok(())
}
