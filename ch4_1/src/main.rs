#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn use_value(_val: i32) {}
fn main() {
    /// primitives possess copy semantics and other types have the move semantics.
    /// that why we can access
    let a = 123;
    use_value(a);
    println!("{}", a);
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c:{:?}", a_status, b_status, c_status);
    // "waiting"...
    // owned data are deleted at the end of the scope.
    // and considering that non primitives only uses move semantics this code 
    // will only works if we return the owned value (sat_a)
    // let a_status = check_status(sat_a);
    // let b_status = check_status(sat_b);
    // let c_status = check_status(sat_c);
    // println!("a: {:?}, b: {:?}, c:{:?}", a_status, b_status, c_status);
}
