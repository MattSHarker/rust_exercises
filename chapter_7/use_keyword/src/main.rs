// demonstrates how to use the "use" keyword to bring specific modules into scope

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// you can also bring enum variants into scope
enum TrafficLight {
    Red,
    Yellow,
    Green
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

use a::series::of;
use TrafficLight::{Red, Yellow};    // how to specify parts of a namespace to use
use Coins::*;   // brings in every variant of the enum

fn main() {
    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    let dime = Dime;
}
