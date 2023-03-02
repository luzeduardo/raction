use regex::Regex;
fn main() {
    let search_term = "picture";
    let haystack = "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. 
    What do we seek through millions of pages?";

    for line in haystack.lines() {
        if line.contains(search_term) {
            println!("contains match: {}", line);
        }
    }

    find_with_regex();
}

fn find_with_regex() {
    let re = Regex::new("picture").unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. 
    What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            //Some(T) is the positive case of an Ooption
            //meaning successful
            Some(_) => println!("regex match: {}", line),
            None => (),
        }
    }
}
