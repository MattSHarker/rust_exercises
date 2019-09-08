use std::collections::HashMap;

fn main() {
    // creating a new empty hash map
    let mut scores = HashMap::new();

    // inserting keys and values into the hash map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 15);
    println!("{:?}", scores);
    // first value is the key, second is the value
    // all keys must be the same type, all values must be the same type

    // you can use the collect method on tuples to create a hash map as well
    let teams = vec![String::from("Green"), String::from("Yellow")];
    let initial_scores = vec![25, 18];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        // the HashMap<_, _> is needed to specify to the collect method which data
        // structure the put the information into
    println!("{:?}", scores);
    
    
    // Objects with the Copy trait will be copied into the hash map
        // everything else will be moved into it
    let field_name  = String::from("Field name");
    let field_value = String::from("Field value");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // from here on, field_name and field_value are invalid


    // accessing values
    println!("\nAccessing individual values: ");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 15);
    
    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name);    // .get() returns an Option<&V>
    println!("{} team's score: {:?}", team_name, team_score);

    // iteration
    println!("\nIteration:");
    for (key, value) in &scores {
        println!("{} team's score: {}", key, value);
    }



    // updating a hashmap

    // overwriting old values
    println!("\nOverwriting vales", );
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 146);
    println!("Original: {:?}", scores);

    // insert a pair with the same key to overwrite
    scores.insert(String::from("Blue"), 213);
    println!("New: {:?}", scores);


    // inserting only if existing key has no value
    println!("\ninserting iff a key has no value");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 20);

    // if there the value for the key DNE, give it the value in or_insert()
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    // updating a value based on the old value
    println!("\nUpdating values based on old values");
    let text = "This has some words that appear once, and words that appear more than once";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // get the value from the key (and set to 0 if value DNE)
        let count = map.entry(word).or_insert(0);

        // modify the value
        *count += 1;
    }

    println!("{:?}", map);
}
