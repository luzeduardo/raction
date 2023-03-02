// to generate an addition overflow due to u8 takes values only until 255
// it is required to add the declarion below.
// otherwise the compiler doesn't finish the compking process 
// because it is an obvious situation
#[allow(arithmetic_overflow)]
fn main() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}
