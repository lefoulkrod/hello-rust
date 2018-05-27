fn main() {
    println!("Hello, world!");
    another_fn();
    let five = five();
    println!("{}", five);
}

fn another_fn() {
    println!("Another fn");
}

fn five() -> i32 {
    5
}