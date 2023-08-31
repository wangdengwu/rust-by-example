mod methods;
mod closures;
mod higher_order_functions;
mod diverging;

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> &'static str {
    return if is_divisible_by(n, 15) {
        "fizzbuzz"
    } else if is_divisible_by(n, 3) {
        "fizz"
    } else if is_divisible_by(n, 5) {
        "buzz"
    } else {
        Box::leak(n.to_string().into_boxed_str())
    };
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_divisible_by() {
        assert_eq!(is_divisible_by(15, 0), false);
        assert_eq!(is_divisible_by(15, 5), true);
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!("fizzbuzz", fizzbuzz(15));
        assert_eq!("fizz", fizzbuzz(3));
        assert_eq!("buzz", fizzbuzz(5));
        assert_eq!("1", fizzbuzz(1));
    }

    #[test]
    fn test_fizzbuzz_to() {
        fizzbuzz_to(15);
    }
}
