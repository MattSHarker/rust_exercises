// This library demonstrates how to use namespaces and the super keyword
// code was taken from the communication crate, to reduce extra coding efforts
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;  // the super keyword references the module one higher than the current one

    // the following would also work, but is less preferable
    // ::client::connect;  // The leading :: references the root module, which is the current one in this case

    #[test]
    fn it_works() {
        client::connect();
    }
}
