use std::collections::HashMap;

fn main() {
    // creating a new empty hash map
    let mut scores = HashMap::new();

    // inserting keys and values into the hash map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 15);
    // first value is the key, second is the value
    // all keys must be the same type, all values must be the same type

    // you can use the collect method on tuples to create a hash map as well
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 15];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        // the HashMap<_, _> is needed to specify to the collect method which data
        // structure the put the information into
    
    

}
