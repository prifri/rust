fn main() {
    let search_term = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and\n\
    dark square is a picture feverishly turned--in search of what?\n\
    It is the same with books.\n\
    Waht do we seek through millions of pages?";

/*
 * prifri, 2022.09.08:
 * - lines()가 반복자를 반환한다고 하며, 이것을 enumerate()와 결함할수있다고 한다.
 * - enumerate()는 반복자 I를 받아 또 다른 튜블 (N, I)를 반환한다.
 *   여기서 N은 0에서 1씩 증가하는 숫자다.
 */
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            println!("{}: {}", i + 1, line);
        }
    }
}
