use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    //creates a file that requires a path and error handling if the file does not exist
    let f = File::open("Cargo.toml").unwrap();
    let mut reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long", line, line.len());
    }
}
