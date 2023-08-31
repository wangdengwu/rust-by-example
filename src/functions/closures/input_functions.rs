// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_functions() {
        // Define a closure satisfying the `Fn` bound
        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);
    }
}