// this is a program to demonstrate ownership and scopes

fn main() {
    let s = String::from("Hello");  // s comes into scope
    takes_ownership(s);             // s's value is now in the function
                                    // s is no longer valid here

    let x = 5;      // x comes into scope
    makes_copy(x);  // the value of x is copied into the function
                    // x is still valid here
}   // x goes out of scope, nothing happens with s as it was already dropped

fn takes_ownership(some_string: String){
    println!("The string: {}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("The int: %", some_int);
}



