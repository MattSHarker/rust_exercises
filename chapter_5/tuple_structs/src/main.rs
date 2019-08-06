// this program demonstrates the basics of tuple structs

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // these are basically named tuples, and act like tuples
    struct Color(i32, i32, i32);    
    struct Point(f32, f32);

    let black  = Color(0, 0, 0);
    let origin = Point(5.5, 8.125);

    println!("Blacks values: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin values: {}, {}", origin.0, origin.1);
}
