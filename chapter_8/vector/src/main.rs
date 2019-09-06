// This program demonstrates the basics of the vector collection

fn main() {
    // how to create a vector
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3]; // when you know what's going into the vector

    // how to add to a vector
    let mut v3 = Vec::new();
    v3.push(1);  // .push(x) adds a value to the vector
    v3.push(2);
    v3.push(3);
    v3.push(4);
    v3.push(5);

    // how to read values from a vector
    let third: &i32 = &v3[2];   // rust is indexed starting at 0
    let fourth: Option<&i32> = v3.get(3);
    println!("third: {}, fourth: {:?}", third, fourth);

    // what happens when a non-existent value is read
    // let does_not_exist: &i32 = &v3[100];    // this will throw an error, good if you don't want the program to continue if too much is accessed
    let does_not_exist: Option<&i32> = v3.get(100);

    // iterating over values in a vector
    let v4 = vec![100, 101, 102, 103, 104];
    for i in &v4 {  // change "&v4" to "&mut v4" to modify values in the vector
        println!("{}", i);
    }
}
