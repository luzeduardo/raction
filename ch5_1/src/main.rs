fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;
    // same bit pattern bbut different types
    println!("a: {:016b}", a); //1100001111000011
    println!("b: {:016b}", b); //1100001111000011
    // uses the same bit pattern to represent two diff numbers. 
    // the type system - not the CPU - is what amkes this distiction
}
