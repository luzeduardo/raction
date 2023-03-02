static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
fn main() {
    let a = 42;
    //references are 4 bytes wide on 32bit CPU and 8 bytes on 64 bit CPU 
    let b = &B;
    let c = &C;
    // {:p} prints the momeory address
    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}
