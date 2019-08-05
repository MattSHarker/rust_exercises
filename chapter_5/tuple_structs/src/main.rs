// this program demonstrates the basics of tuple structs

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // these look like tuples but function as structs
    struct Color(i32, i32, i32);    
    struct Point(i32, i32, i32);

    let black  = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
