fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    send_slice(hello);
}

fn send_slice(slice: &str) {
    println!("{}", slice);
}