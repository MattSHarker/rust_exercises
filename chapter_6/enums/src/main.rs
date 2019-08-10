// a program to demonstrate enums

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // enums can contain any combo of types
    V6(String),
}

#[derive(Debug)]
enum IpAddr_2 {
    V4(IpAddr), // they can also store enums
    V6(IpAddr),
}

// methods can be created for enums in the same way as structs
impl IpAddr_2 {
    fn print_ip_method(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    // setting up enums
    let home     = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // retreiving values manually via pattern matching
    match home {
        IpAddr::V4(val1, val2, val3, val4) =>
                println!("Home ip: {}:{}:{}:{}", val1, val2, val3, val4),   // pattern matching
            _ => println!("Unexpected format"),
    }

    // setting up enums using enums
    let home_2     = IpAddr_2::V4(home);
    let loopback_2 = IpAddr_2::V6(loopback);

    // retreiving values with if-let
    if let IpAddr_2::V6(IpAddr::V6(ip)) = &loopback_2 {
        println!("loopback (v2): \"{}\"", ip);
    }
    
    // putting enums into functions
    print_ip(&home_2);
    loopback_2.print_ip_method();
}

// functions can take in each type of an enum without needing to modify much
fn print_ip(ip: &IpAddr_2) {
    println!("ip: {:?}", ip);
}
