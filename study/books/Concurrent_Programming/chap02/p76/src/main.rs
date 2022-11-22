/*
 * prifri, 2022.11.22:
 * - Add trait import
 */

use std::ops::Add;

struct Vec2 {
    x: f64,
    y: f64
}

impl Add for Vec2 {
    type Output = Vec2;

/*
 * prifri, 2022.11.22:
 * - + trait.
 */
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let v1 = Vec2{x: 10.0, y: 5.0};
    let v2 = Vec2{x: 3.1, y: 8.7};
/*
 * prifri, 2022.11.22:
 * - 여기서 add가 호출된다.
 * - v1, v2의 소유권이 여기서 v로 이동했으므로 이제 v1, v2는 사용못한다.
 */
    let v = v1 + v2;
    //println!("{} {}", v1.x, v2.x);
    println!("v.x = {}, v.y = {}", v.x, v.y);
}
