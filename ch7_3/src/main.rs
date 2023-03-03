// import traits to heavily work with RW io operations
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#";
fn main() {
    println!("Hello, world!");
}"#;

fn main() -> std::io::Result<()> {
    //makes space for the programs input with an internal buffer
    let mut buffer: Vec<u8> = vec!();
    // reads our input and inserts it into our internal buffer
    INPUT.read_to_end(&mut buffer)?;

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        // writes the current position with up to 8 left-padded zeros
        print!("[0x{:08x}] ", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
