fn main() {
    let mut s1 = String::from("hello");
    
    {
        let r1 = &s1;
        borrow_test(r1);
    }

    { 
        let r2 = &mut s1;
        mutate(r2);
    }
    println!("mutated {}", s1);
}

fn borrow_test(s1: &String) {
    println!("borrowed {}", s1)
}

fn mutate(s1: &mut String) {
    s1.push_str(", world");
}