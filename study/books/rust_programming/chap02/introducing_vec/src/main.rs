fn main() {
    let context_lines = 2;
    let needle = "oo";
    let haystack = "\
    Every face, every shop,\n\
    bedroom window, public-house, and\n\
    dark square is a picture\n\
    feverishly turned--in search of whatt?\n\
    It is the same weith books.\n\
    What do we seek\n\
    through millions of pages?\n";
    
/*
 * prifri, 2022.09.09:
 * - line 번호 - 1이 저장될것.
 */
    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(
        usize, String)>> = Vec::new();

/*
 * prifri, 2022.09.09:
 * - line단위로 iterate.
 * - i는 for idx가 저장될것.
 */
    for (i, line) in haystack.lines().enumerate() {
        if !line.contains(needle) {
            continue;
        }

/*
 * prifri, 2022.09.09:
 * - needle이 있는 line이라면 현재 iter idx를 저장하고
 */
        tags.push(i);

/*
 * prifri, 2022.09.09:
 * - vec size를 에약하여 생성하고 ctx에 넣어놓는다.
 */
        let v = Vec::with_capacity(2 * context_lines + 1);
        ctx.push(v);
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {

/*
 * prifri, 2022.09.09:
 * - saturating_sub : 뺄셈을할때 정수가 0보다 작아지면 0을 반환.
 *   context_lines 이내의 line들을 저장한다.
 */
            let lower_bound =
                tag.saturating_sub(context_lines);
            let upper_bound =
                tag + context_lines;

            if (i < lower_bound) | (upper_bound < i) {
                continue;
            }

/*
 * prifri, 2022.09.09:
 * - 범위내라면 line string을 copy후 local_ctx i 번호로에 넣는다.
 */
            let line_as_string = String::from(line);
            let local_ctx = (i, line_as_string);

/*
 * prifri, 2022.09.09:
 * - j가 i의 ctx index에 해당하므로 거기에 만든 ctx로 새로 고친다.
 */
            ctx[j].push(local_ctx);
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
