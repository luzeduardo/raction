
fn main() {
    let a: i64 = 42;
    //casts a reference &a to a constant raw pointeri64(*const i64)
    let a_ptr = &a as *const i64;

    println!("a: {} ({:p})", a, a_ptr);
}
