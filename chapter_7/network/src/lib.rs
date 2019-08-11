// demonstrates how to make a library

mod network {
    fn connect() {

    }

    // modules can be inside other modules
    mod client {    // doesn't clash with the other client module as this is network::client
        fn connect() {

        }
    }
}

// there can be multiple modules...
mod client {
    // and those modules can share function names
    fn connect() {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
