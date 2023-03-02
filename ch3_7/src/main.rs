use rand::{random};
static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}
fn main() {
    println!("Hello, world!");
}
