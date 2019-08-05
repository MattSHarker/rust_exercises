// This program demonstrates the basic properties of structs

struct User {
    username:      String,
    email:         String,
    sign_in_count: u64,
    active:        bool,
}

fn main() {
    // initialize an object of the User class
    let mut user0 = User {
        email:         String::from("Someone@email.com"),
        username:      String::from("Someusername123"),
        active:        true,
        sign_in_count: 1,
    };  // remember the trailing semi-colon


    // retreive and overwrite information
    // !! must be mut to do this !!
    println!("Original email: {}", user0.email);
    user0.email = String::from("anotheremail@example.com");
    println!("Modified email: {}", user0.email);


    // use info from user0 to help create user1 and user2
    let user1 = User {
        email: String::from("Another@Email.com"),
        username: String::from("UserName"),
        active: user0.active,
        sign_in_count: user0.sign_in_count,
    };

    let user2 = User {
        email: String::from("AnotherAnother@Email.com"),
        username: String::from("UserNameUserName"),
        ..user0 // this says to fill the rest of the forms the same as they are for user0
    };
}