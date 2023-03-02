fn main() {
    let search_term = "picture";

    let quote = "\
    Every face, every shop, bedroom window,
    public-house, and dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do ew seek through millions of pages?
    ";

    //lines() returns a iterator over quote
    //  where each iteration is a line of text
    //because lines return an iterator it can be chained with enumerate
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            println!("{}: {}", i + 1, line);
        }
    }
}
