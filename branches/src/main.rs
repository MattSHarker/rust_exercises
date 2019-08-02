// a file to experiment with control flow

fn main() {
    println!();
    basic_if(3);
    basic_if(6);

    println!();
    else_if(4);
    else_if(5);
    else_if(6);

    println!();
    if_in_let(true);
    if_in_let(false);


}

// demonstrates how to use a simple if/else
fn basic_if(x: i32) {
    if x < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

// demonstrates how to use an if/else chain
fn else_if(x: i32) {
    if x < 5 {
        println!("x is less than 5 ({})", x);
    } else if 5 < x {
        println!("x is greater than 5 ({})", x);
    } else {
        println!("x is equal to 5 ({})", x);
    }
}

// demonstrates how to use a conditional in a let
fn if_in_let(condition: bool) {
    let number = if condition {
        5
    } else {
        6
    };

    println!("Since the bool is {}, the number is {}", condition, number);
}
