fn main() {
    // mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadow
    let shadow = 0b0000_1111;
    println!("The value of shadow is: {}", shadow);
    let shadow = 'a';
    println!("The value of shadow is: {}", shadow);

    // tuples
    let tup = ('a', 0b00001111, 25.6);
    let (f, s, t) = tup;
    println!("The value of f is: {}", f);
}