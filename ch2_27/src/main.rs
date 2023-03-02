use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    //creates a file that requires a path and error handling if the file does not exist
    let f = File::open("Cargo.toml").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break
        }
        println!("{} ({}) bytes long", line, len);
        // shrink the string back to length 0
        // prevents line from persist into the following ones
        line.truncate(0); 
    }
}
