#[derive(Debug, Clone)]
struct CubeSat {
    id: u64,
}

#[derive(Copy,Clone,Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}
fn main() {
    let sat_a = CubeSat { id: 1 };

    let a_status = check_status(sat_a.clone());
    println!("a: {:?}", a_status);

    // this works due to CubeSat implements clone. So we are not moving it to check_status scope
    println!("{:?}", sat_a);
}
