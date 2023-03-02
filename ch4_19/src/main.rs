use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {}

fn main() {
    /// Rc enables multiple onwership explicity keeping track of number of references to a value 
    /// to determine if still in use.
    /// Thinking as a TV and multiple watchers: the TV will only be turned off after rhe last watcher left the room
    /// it is only for single-threaded scenarios
    let base = Rc::new(GroundStation {});

    println!("{:?}", base);
}
