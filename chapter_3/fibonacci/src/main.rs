// displays the xth element of the fibonacci sequence
    // using two methods (iterative and recursive)
// first element starts at 0 (0, 1, 1, 2, 3, 5...)

fn main() {
    let x = 10;

    println!("Iterative:");
    println!("The {} element of the fibonacci sequence is: {}", x, fib_itr(x));

    println!("\nRecursive:");
    println!("The {} element of the fibonacci sequence is: {}", x, fib_rec(x));
}

// iterative method to generate the xth number
fn fib_itr(x:i32) -> i32 {
    let mut prev_prev;
    let mut prev = 0;
    let mut cur = 1;

    for _number in 1..x-1 {
        prev_prev = prev;
        prev = cur;
        cur = prev + prev_prev;
    }

    cur
}

// recursive method to generate the xth number
fn fib_rec(x: i32) -> i32 {
    if x == 2 {
        1
    } else if x == 1 {
        0
    } else {
        fib_rec(x-1) + fib_rec(x-2)
    }
}
