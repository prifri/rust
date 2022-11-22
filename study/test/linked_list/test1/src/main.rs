/*
 * prifri, 2022.11.22:
 * - 원본
 *   https://dev.to/felixfaisal/implementing-linked-list-in-rust-3and
 *
 * - 재귀로 구현되있다.
 *   꼬리 재귀를 사용하는데, 검색결과 rust는 꼬리재귀에 대해 완벽한 지원을
 *   안하고 있다.
 *   https://stackoverflow.com/questions/59257543/when-is-tail-recursion-guaranteed-in-rust
 *   https://dev.to/seanchen1991/the-story-of-tail-call-optimizations-in-rust-35hf
 *   그렇다고 loop로 바꾸자니 쉽지가 않다..
 *
 * - 또한 head를 지울방법이 없다. value = 0 으로하여 암묵저으로 지우지만
 *   이건 그냥 적당히 한거일뿐..
 */

#[derive(Clone)]
enum Address {
    Address(Box<List>),
    Nil,
}

#[derive(Clone)]
struct List {
    value: u32,
    next: Address,
}

impl List {
    fn append(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                next_address.append(elem)
            }
            Address::Nil => {
                let node = List {
                    value: elem,
                    next: Address::Nil
                };
                self.next = Address::Address(Box::new(node))
            }
        }
    }

    fn delete(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                if next_address.value == elem {
                    println!("Deleting value {}", next_address.value);
                    self.next = next_address.next.clone();
                } else {
                    next_address.delete(elem);
                }
            }
            Address::Nil => {
                return
            }
        }
    }

    fn print(&mut self) {
        match self.next {
            Address::Address(ref mut next_address) => {
                print!("{} -> ", self.value);
                next_address.print();
            }
            Address::Nil => {
                println!("{}", self.value);
                return
            }
        }
    }
}

fn main() {
    let mut head = List {
        value: 0,
        next: Address::Nil,
    };
    head.append(1);
    head.append(2);
    head.append(3);
    head.append(4);
    head.append(5);
    head.append(6);
    head.append(7);
    head.print();
    head.delete(3);
    head.print();
}
