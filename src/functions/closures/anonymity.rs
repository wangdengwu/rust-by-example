// `F` must be generic.
fn fn_once<F>(f: F) where
    F: FnOnce() {
    f();
}

// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

#[cfg(test)]
mod tests {
    use super::{apply, fn_once};

    #[test]
    fn test_anonymity() {
        let x = 7;

        // Capture `x` into an anonymous type and implement
        // `Fn` for it. Store it in `print`.
        let print = || println!("{}", x);

        apply(print);
        fn_once(print);
    }
}