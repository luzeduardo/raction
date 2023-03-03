use std::fs::File;
use std::io::prelude::*;
use std::env;

const BYTES_PER_LINE: usize = 16;
fn main() {
    // handle command line args without 3rd libs
    // it returns an iterator with an nth method that extract the nth position
    // every iterator  nth returns an Option
    let arg1 = env::args().nth(1);

    // the expect is considered a friendlier version of unwrap
    // it takes an error message as arument whereas unwrap simply panics
    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    //goes out of loop when f.read_exact 
    //returns Err when it run out of bytes to read
    // read_exact from Read trait transfers data from the source to the buffer
    // provided as argument. it stops when the buffer is full
    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }
}
