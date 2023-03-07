use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::io::{self};
#[derive(Debug)]
pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

type ByteString = Vec<u8>;
// type ByteStr = [u8];

#[derive(Debug)]
pub struct KeyvaluePair {
    pub key: ByteString,
    pub valur: ByteString,
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;
        Ok(ActionKV {
            f: f,
            index: HashMap::new(),
        })
    }

    pub fn get() {}
    pub fn load(&mut self) -> io::Result<()> {
        // let mut f = BufReader::new(&mut self.f);
        // loop {
        //     let position = f.seek(SeekFrom::Current(0))?;
        //     let maybe_kv = ActionKV::process_record(&mut f);
        //     let kv = match maybe_kv {
        //         Ok(kv) => kv,
        //         Err(err) => {
        //             match err.kind() {
        //                 io::ErrorKind::UnexpectedEof => {
        //                     break;
        //                 }
        //                 _ => return Err(err),
        //             }
        //         }
        //     };
        //     self.index.insert(kv.key, position);
        // }
        Ok(())
    }
    pub fn delete() {}
    pub fn update() {}
    pub fn insert() {}
}