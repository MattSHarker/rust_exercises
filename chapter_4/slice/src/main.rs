// Demonstrates the slice type

fn main() {
    println!("~~~ Basic Concept ~~~");
    basic_concept();

    println!("\n~~~ First Word ~~~");
    let s = String::from("This is a series of words");
    let first = first_word(&s);
    println!("The first word of the string is: {}", first);
    
    println!("\n~~~ Array Slice ~~~");
    arr_slice();
}

// basic concept of a slice
fn basic_concept() {
    let hw = String::from("world hello");
    let world = &hw[0..5];
    let hello = &hw[6..11];
//  let world = &hw[..5];  // these can also be written as such, since 0 is the very
//  let hello = &hw[6..];  // first element, and 11 is the very last element

    println!("{} {}", hello, world);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn arr_slice() {
    let arr = [1, 2, 3, 4, 5];
    let slc = &arr[1..4];

    println!("Original array: {:?}", arr);  // {:?} allows arrays and structs to be printed
    println!("Array slice: {:?}", slc);
}
