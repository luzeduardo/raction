//reference counting doesnt allow mutation 
// it is not thread-safe
use std::rc::Rc; 
//so we use a reference cycle to wrap it and let the internal values to 
//be modified while it presents an immutable façade
use std::cell::RefCell; 

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64 //Mhz
}
fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(GroundStation { radio_freq: 98.31 }));
    println!("base: {:?}", base);

    {
        // new scope where base can be mutably borrowed
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 10.31;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    // borrow_mut mutabbly borrows the wrapped value
    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 10.34;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
