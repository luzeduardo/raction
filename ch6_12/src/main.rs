fn main() {
    // this program tries to read a an address in memory that is not entitled
    // and it generates a segmentation fault
    // and it occurss because we i'm trying to scan through bytes starting from 1

    //a segmentation fault is raised by the CPU when an illegal memory address is requested
    let mut n_nonzero = 0;

    //starting from 0 will raise a null pointer
    for i in 1..10000 {
        //converts to a raw pointer of type u8 to inspect raw memomry addressess
        let ptr = i as *const u8;
        //reads the value at address i.
        //another way to say it is: 
        //read the value being pointed to
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }
    println!("non-zero bytes in memory: {}", n_nonzero);
}
