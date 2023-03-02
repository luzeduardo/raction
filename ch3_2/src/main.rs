#[derive(Debug)]
struct File {
    name: String,
    // Vec<u8> provides dynamic sizing, to simulate writing to a file
    data:  Vec<u8>,
}
struct Hostname (String);
fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    //acessing by reference prevents their use after move issues
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);

    let h = Hostname(String::from("localhost"));
    connect(h);
}


fn connect(host: Hostname) {
    println!("{}", host.0);//access tuple data with index
}