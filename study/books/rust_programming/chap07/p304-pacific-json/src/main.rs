
/*
 * prifri, 2022.10.06:
 * - 책내용대로하면 잘안되서 그냥 이렇게 쓴다..
 */
use serde_json::json;

fn main() {

/*
 * prifri, 2022.10.06:
 * - json!
 *   JSON 리터럴과 러스트의 표현식을 사용하여 String값을 구현한다.
 *   이 값을 json 명세 내의 모든 타입을 나타낼 수 있는 열거형인
 *   serde_json::Valuetype의 러스트 값으로 변홚나다.
 */
    let capitals = json!({
        "Cook Islands": "Avarua",
        "Fiji": "Suva",
        "Kiribati": "South Tarawa"
    });

    println!("Capital of tonga is: {}", capitals["Fiji"]);
    println!("Capital of ABC is: {}", capitals["ABC"]);
}
