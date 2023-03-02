#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id.id, StatusMessage::Ok);
    //returning ownership back
    sat_id
}
fn main() {
    let sat_a = CubeSat { id: 1 };
    //moving ownership into function local variable
    let sat_a = check_status(sat_a);
    //ownership movement
    let sat_a = check_status(sat_a);
}
