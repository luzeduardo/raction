fn mock_rand(n: u8) -> f32 {
    (n as f32) / 255.0 // 255 is the max value that u8 can represent
}
fn main() {
    println!("{}", mock_rand(2));
}
