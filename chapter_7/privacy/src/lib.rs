// this is a program to test out the differences between public and private
// functions and modules

mod outermost{
    pub fn middle_function() {}
    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}
        fn secret_function() {}
    }
}

fn try_me_out() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
