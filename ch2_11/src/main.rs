fn main() {
    let needle = 15;
    let haystack = [1, 1,2,5,15,52,2203, 877, 4140, 21147];

    for item in &haystack { //iterates over refs to elements
        if *item == needle { // the *item returns the item ref
            println!("{}", item);
        }
    }
}

/*
- <'a, 'b> declares two lifetime vars within scope 
    of add_with_lifetimes. These are normally spoken as lifetime a and b

- i: &'a i32 binds lifetime var to 'a
*/
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}


// it says that T must implement std::aop::Add
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}