// This is a program to show how a struct could be
// used in an actual program

fn main() {
    no_structs();
    with_tuples();
    with_structs()
}

// This is how an area would be calculated without structs or tuples
// Makes it difficult to have multiple sizes
fn no_structs() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square units.",
        area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}



// This is how it can be done using a tuple
fn with_tuples() {
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square units.",
        area_tuple(rect)
    )
}

// An area function that takes in a tuple
fn area_tuple(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}



// How it can be done with structs
struct Rectangle {
    width:  u32,
    height: u32,
}

fn with_structs() {
    let rect = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square units.",
        area_struct(&rect)
    )
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
