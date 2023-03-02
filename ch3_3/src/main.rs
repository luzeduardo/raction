#![allow(unused_variables)]

#[derive(Debug)]

struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str, data: Vec<u8>) -> File {
        File { name: String::from(name), data: data }
    }
}
fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

//returns the number of bytes to read
fn read(f:  &File, save_to: &mut Vec<u8>) -> usize {
    // makes a copy due to save_to.append shrinks the input
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    //ensure there is space to fit incoming data
    save_to.reserve(read_length);
    // allocates sufficient data in save_to buffer to hold the contents of f
    save_to.append(&mut tmp);
    read_length
}
fn main() {
    let mut f2 = File::new("3.txt", [114, 117, 115, 116, 33].to_vec());

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f2);
    println!("{} is {} bytes length", &f2.name, f2_length);
    println!("{}", text);
}
