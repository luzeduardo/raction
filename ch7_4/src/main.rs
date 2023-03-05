use std::path::PathBuf;
fn main() {
    let hello = String::from("/tmp/hello.txt");
    let tmp_dir = hello.split("/").nth(0);
    println!("{:?}", tmp_dir); //prints Some("")

    // Path provide dedicated methods for common operations such as setting a file etension
    let mut path_buff_hello = PathBuf::from("/tmp/hello.txt");
    path_buff_hello.pop();
    println!("{:?}", path_buff_hello.display());
}
