// demonstrates basic if let capabilities

fn main() {
    let some_u8_val = Some(0u8);

    // execute only if the value is 3
    match some_u8_val {
        Some(3) => println!("Three"),
        _       => (),
    }

    // same as above but with if let
    if let Some(0u8) = some_u8_val {
        println!("Zero u eight");
    }

    // else can be used to a similar effect as _
    if let Some(16) = some_u8_val {
        println!("Sixteen");
    } else {
        println!("Not sixteen");
    }
}
