// A program to demonstrate the Option enum, as well as some of its methods

// pub enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    // vars using the Some type
    let some_number = Some(5);
    let some_string = Some("A string");

    // var using the None type
    let absent_number: Option<i32> = None;
    let absent_string: Option<&str> = None;

    // is_some() method returns true if Some, false if None
        // reverse for is_none()
    assert_eq!(some_number.is_some(), true);    // throws error if LHS != RHS
    assert_eq!(some_number.is_none(), false);

    // expect() compares the contents of the Option<&str> against a string
        // if None, panic and display string in expect()
    assert_eq!(some_string.expect("Panic Message Here"), "A string");
//     assert_eq!(absent_string.expect("Panic Message Here"), "A third string");   // uncomment for panic example

    // unwrap(self) moves the value v out of Option<T> if it is Some(v)
    assert_eq!(some_number.unwrap(), 5);

    // replace()
    let mut x = Some(2);
    let old = x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    // copied() copies contents of one Option to another (see also: cloned())
    let mut x = 12;
    let opt_x = Some(&x);
    let copied = opt_x.copied();
    assert_eq!(opt_x, Some(&12));
    assert_eq!(copied, (Some(12)));
}
