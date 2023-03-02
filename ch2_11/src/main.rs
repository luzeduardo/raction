fn main() {
    let needle = 15;
    let haystack = [1, 1,2,5,15,52,2203, 877, 4140, 21147];

    for item in &haystack { //iterates over refs to elements
        if *item == needle { // the *item returns the item ref
            println!("{}", item);
        }
    }
}
