fn main() {
    let a : i32 = 40; //lives on the stack
    let b: Box<i32> = Box::new(30); //lives on the heap
    println!("{} + {} = {}", a, b, a + *b); //to access b value it is required to dereference it
}
