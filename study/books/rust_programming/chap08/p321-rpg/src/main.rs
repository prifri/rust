/*
 * PRIFRI, 2022.10.06:
 * - rand::Rng.
 *   trait
 * - rand::seq::ThreadRng
 *   구조체
 */
use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

/*
 * PRIFRI, 2022.10.06:
 * - trait 객체. 제네릭이랑 비슷한데 pointer 개념이라 runtime비용이 좀 있다는듯.
 */
trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
/*
 * PRIFRI, 2022.10.06:
 * - gen_bool
 *   인자에 따라 bool값 생성. 0.5면 50%확률.
 */
        let spell_is_successful = rand::thread_rng()
            .gen_bool(probability_of_success);

        println!("{:?} mutters incoherently. ", self);
        if spell_is_successful {
            println!("The {:?} glows brightly.", thing);
        } else {
            println!("The {:?} fizzes, \
            then turns into a worhless trinket.", thing);
            *thing = Thing::Trinket {};
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}

fn main() {
    let mut it = Thing::Sword;

    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    let party: Vec<&dyn Enchanter> = vec![&d, &h, &e];
/*
 * PRIFRI, 2022.10.06:
 * - choose로 party중 하나가 임의로 선택된다.
 *   
 */
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();
/*
 * PRIFRI, 2022.10.06:
 * - 드워프로 따졋을때. d.enchant(&mut it); 의 개념이 된다.
 */
    spellcaster.enchant(&mut it);
}
