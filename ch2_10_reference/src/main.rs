fn main() {
    let a = 42;
    // r is reference to a
    let r = &a; 
    // add a + a( via dereferencing r) 
    // and assigns it to b
    let b = a + *r;

    println!("a + a = {}", b);
}
