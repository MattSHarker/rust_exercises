// converts temperatures between Fahrenheit and Celcius

fn main() {
    let temp = 51.5;
    println!("{}F is {}C", temp, f_to_c(temp));
    println!("{}C is {}F", temp, c_to_f(temp));
}

// converts Fahrenheit to Celcius
fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

// converts Celcius to Fahrenheit
fn c_to_f(c: f64) -> f64 {
    c * (9.0/5.0) + 32.0
}
