// demonstrates the basic concept of match

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // and so on ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);

    let some_u8_val = 0u8;
    match some_u8_val {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),    // _ matches everything, () does nothing
    }
}

// takes in a Coin and returns the value (and prints the state if a quarter)
fn coin_value(coin: Coin) -> u32 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter(state) => {
            println!("Quarter is from {:?} state", state);
            25
        },
    }
}

// takes in an Option<i32> and returns an Option<i32> that is 1 more
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
