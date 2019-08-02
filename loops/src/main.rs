// a program to experiment with loops

fn main() {
    println!();
    while_loop();

    println!();
    for_loop_clctn();

    println!();
    for_loop_range();
}

// demonstrates an infinite loop (ctrl-c to end program)
// commented out to avoid compiler warnings
// fn inf_loop() {
//     loop {
//         println!("Loop!");
//     }
// }

// demonstrates a while loop
fn while_loop() {
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value of index {} is: {}", index, arr[index]);

        index = index + 1;
    }
}

// demonstrates a for loop using a Collection
fn for_loop_clctn() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("The value is: {}", element);
    }
}

// demonstrates a for loop using a Range
fn for_loop_range() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
