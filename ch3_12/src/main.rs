#![allow(unused_variables)]

use std::vec;

#[derive(Debug)]

struct File;
trait Read {
    fn read(
        // the pseudo-type Self is a placeholder for the type that eventually implements trait Read
        self: &Self,
        save_to: &mut Vec<u8>,
    ) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0) //stub that complies with trait
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec!();
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} bytes read from {:?}", n_bytes, f);
}
