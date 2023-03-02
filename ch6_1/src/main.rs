//returns size of type in bytes
use std::mem::size_of; 

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
fn main() {
    let a: usize= 42; // usize is the memory address size for the CPU the code is compiled for
    
    //references are 4 bytes wide on 32bit CPU and 8 bytes on 64 bit CPU 
    //a reference to an array of 10 bytes. the reference is placed on the stack
    let b: &[u8; 10] = &B;

    //the Box type is a boxed byte slice. 
    //when place values inside a box, ownership of the value moves to the owner of the box
    let c: Box<[u8]> = Box::new(C);
    // {:p} prints the momeory address
    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
    println!();
    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size: {:?} bytes", size_of::<usize>());
    println!("  value: {:?}", a);
    println!();

    println!("b: (a reference to B):");
    println!("  location: {:p}", &b);
    println!("  size: {:?} bytes", size_of::<&[u8; 10]>());
    println!("  points to: {:?}", b);
    println!();

    println!("c: (c 'box' for C):");
    println!("  location: {:p}", &c);
    println!("  size: {:?} bytes", size_of::<Box<[u8]>>());
    println!("  points to: {:?}", c);
    println!();

    println!("B: (an array of 10 bytes):");
    println!("  location: {:p}", &B);
    println!("  size: {:?} bytes", size_of::<[u8; 10]>());
    println!("  value: {:?}", B);
    println!();

    println!("C: (an array of 11 bytes):");
    println!("  location: {:p}", &C);
    println!("  size: {:?} bytes", size_of::<[u8; 11]>());
    println!("  value: {:?}", C);
    println!();
}
