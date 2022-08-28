fn main() {
    let mut letters = vec![
        "a", "b", "b"
    ];


/*
 * IAMROOT, 2022.08.28:
 * - for 문 밖에서는 가능
 */
    letters.push(letters[1]);

/*
 * IAMROOT, 2022.08.28:
 * - for문안에서 변경을 하는건 rust에서 막는다
 */
    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone());
    }
}
