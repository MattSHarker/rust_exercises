// demonstrates basic properties of references and borrowing
fn main() {
    let s0 = String::from("This is a looooooooooooooooooong string");
    let len = calculate_length(&s0);
    println!("The length of  [{} is {} characters long", s0, len);


    // mutable references
    let mut s1 = String::from("This string");
    change(&mut s1);
    println!("{}", s1);

    // multible non-mutable references
    let mut s2 = String::from("Boop");
    let r1 = &s2;   // you can have any number of immutable references
    let r2 = &s2;       // but only one mutable reference (with 0 immutable refs)
    println!("r1: {}, and r2: {} both reference s1: {}", r1, r2, s1);
}

// demonstrates the basic use of references
fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

// demonstrates mutable references
fn change(some_string: &mut String) {   // both the argument and the parameter must be mutable
    some_string.push_str(" has been modified");
}
