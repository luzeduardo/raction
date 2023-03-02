use std::mem::drop;
fn main() {
    let a: Box<i32> = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a +*b + *c;
    drop(a); //frees the memory for another use
    let d = Box::new(1);
    let result2 = *b + *c + *d;
    println!("{} {}", result1, result2);
}
