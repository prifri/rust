fn main() {
    let search_term = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and\n\
    dark square is a picture feverishly turned--in search of what?\n\
    It is the same with books.\n\
    Waht do we seek through millions of pages?";
    let mut line_num: usize = 1;

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}: {}", line_num, line);
        }
        line_num += 1
    }
}
