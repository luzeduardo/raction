fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through million of pages?";

    let mut lines_that_match: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            lines_that_match.push(i);
            // with_capacity reserves space for n items
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if lines_that_match.is_empty() {
        return;
    }

    //iterate over content
    for (i, line) in haystack.lines().enumerate() {
        //iterate over lines that match the needle
        for (j, tag) in lines_that_match.iter().enumerate() {
            //saturating_sub is substraction that returns 0 on integer underflow
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        // ref line informs the compiler that we want to borrow
        // this value rather than move it
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
