fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    //interprets a *const i64 as usize using transmute that is hightly unsafe
    let a_addr: usize = unsafe {
        std::mem::transmute(a_ptr)
    };

    println!("a: {} ({:p}) ...0x{:x}", a, a_ptr, a_addr + 7);
}
