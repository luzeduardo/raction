//! Simulating files one step at a time.
#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display};

#[derive(Debug,PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// Represents a "file",
/// which probably lives in a file system
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    ///Creates a new, empty `File`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Returns the file's length in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Return the file's name
    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"))
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
           FileState::Open => write!(f, "OPEN"),
           FileState::Closed => write!(f, "CLOSED"), 
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}
fn main() {
    let mut f5 = File::new("5.txt");

    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{}", f5); //uses own Display impl

    println!("{} is {} bytes long", &f5.name(), f5.len());
    println!("{}", text);
}
