struct Foo {
    val: u32
}

fn main() {
    let mut x = Foo{val: 10};
    {
        let a = &mut x;
        println!("a, val = {}", a.val);
/*
 * prifri, 2022.11.22:
 * - a에 x를 대여중이므로 error.
 * - x는 busy상태라 봐도 될듯?
 */
        //println!("x.val = {}", x.val);
        let b: &Foo = a;
        println!("b. val = {}", b.val);
/*
 * prifri, 2022.11.22:
 * - 여기서 b가 해제
 */
        a.val = 30;
    }
    {
        let c = &x;
        println!("c.val = {}", c.val);
        println!("x.val = {}", x.val);
/*
 * prifri, 2022.11.22:
 * - 위와 같이 x는 c에 대여중(busy)이므로 d로 못가져온다
 */
        //let d = &mut x;
        //d.val = 40;
        println!("c.val = {}", c.val);
    }

    println!("x.val = {}", x.val);
}
