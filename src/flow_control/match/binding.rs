// A function `age` which returns a `u32`.
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_primitives() {
        println!("Tell me what type of person you are");
        let result =
            match age() {
                0 => format!("I haven't celebrated my first birthday yet"),
                // Could `match` 1 ..= 12 directly but then what age
                // would the child be? Instead, bind to `n` for the
                // sequence of 1 ..= 12. Now the age can be reported.
                n @ 1..=12 => format!("I'm a child of age {}", n),
                n @ 13..=19 => format!("I'm a teen of age {}", n),
                // Nothing bound. Return the result.
                n => format!("I'm an old person of age {}", n),
            };
        assert_eq!(result, "I'm a teen of age 15");
    }

    #[test]
    fn test_binding_option() {
        let result =
            match some_number() {
                // Got `Some` variant, match if its value, bound to `n`,
                // is equal to 42.
                Some(n @ 42) => format!("The Answer: {}!", n),
                // Match any other number.
                Some(n) => format!("Not interesting... {}", n),
                // Match anything else (`None` variant).
                _ => unreachable!(),
            };
        assert_eq!(result, "The Answer: 42!");
    }
}
