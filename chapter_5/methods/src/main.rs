// This demonstrates how to use methods (functions in structs)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // calculates the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // checks if both dims of another rectangle are smaller 
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // an associated function to create a square (side * side)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // create Rectangle objects and display their contents
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 25, height: 40 };
    let rect3 = Rectangle::square(45);  // associated function

    println!("rect1: {:?}", rect1);
    println!("rect2: {:?}", rect2);
    println!("rect3: {:?}", rect3);

    // demonstrate the Rectangle::area method
    println!();
    println!("The area of rect1 is {}", rect1.area());
    println!("The area of rect2 is {}", rect2.area());
    println!("The area of rect3 is {}", rect3.area());

    // demonstrate the Rectangle::can_hold method
    println!();
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //

}
