fn main() {
    let bank_account = 400;

    let emotion = if bank_account < 500 {
        "sad"
    } else {
        "happy"
    };

    println!("emotion is {}", emotion);

    for_in();

    range();
}

fn for_in() {
    let a = [1,2,3,4,5,6];
    for val in a.iter() {
        println!("val is {}", val);
    }
}

fn range() {
    for a in (1..5).rev() {
        println!("{}", a);
    }
}