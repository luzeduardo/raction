#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}
#[allow(dead_code)]
//the ! return type indicates the compiler this fn never returns
fn read(f &mut File, save_to: &mut Vec<u8>) -> ! {
    // a macro that crashes the program if encountered
    unimplemented!() 
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(f1, vec![]);
    close(&mut f1);
}