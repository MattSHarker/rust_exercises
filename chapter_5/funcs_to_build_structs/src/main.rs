// This program demonstrates how to use functions to build structs

#[derive(Debug)]    // !! ignore this, it's only being used to print the struct !!
struct User {
    username:      String,
    email:         String,
    sign_in_count: u64,
    active:        bool,
}

fn main() {
    println!("\n~~~ Init with Function ~~~");
    let user3_email = String::from("user2@email.com");
    let user3_name  = String::from("User2Name");
    let user3 = build_user(user3_email, user3_name);
    println!("User2 info: {:?}", user3);
}

// basic way to use a function to create a new user
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// same as above, but since the parameter names and struct fields have the
// same name, field init shorthand can be used
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email,      // this line...
        username,   // and this line are in shorthand
        active: true,
        sign_in_count: 1,
    }
}

