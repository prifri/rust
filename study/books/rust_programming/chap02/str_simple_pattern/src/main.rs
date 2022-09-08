
/*
 * prifri, 2022.09.08:
 * - String        str
 *   Vec<u8>       [u8]
 *   소유된 타입.  대여된 타입
 *   read/write    read only
 *   저성능        고성능
 *                 보통 &str로 쓰임. 포인트만 가져다쓰는개념이라 그럼.
 *                 문자열 리터럴. &'static str
 *
 */
fn main() {
    let search_term = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and\n\
    dark square is a picture feverishly turned--in search of what?\n\
    It is the same with books.\n\
    Waht do we seek through millions of pages?";

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}
