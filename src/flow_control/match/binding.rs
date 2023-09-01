// A function `age` which returns a `u32`.
type Age = fn() -> u32;

fn age(a: Age) -> String {
    match a() {
        0 => format!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => format!("I'm a child of age {}", n),
        n @ 13..=19 => format!("I'm a teen of age {}", n),
        // Nothing bound. Return the result.
        n => format!("I'm an old person of age {}", n),
    }
}

type SomeNumber = fn() -> Option<u32>;

fn some_number(sn: SomeNumber) -> String {
    match sn() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => format!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => format!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_primitives() {
        println!("Tell me what type of person you are");
        assert_eq!(age(|| 0), "I haven't celebrated my first birthday yet");
        assert_eq!(age(|| 12), "I'm a child of age 12");
        assert_eq!(age(|| 15), "I'm a teen of age 15");
        assert_eq!(age(|| 25), "I'm an old person of age 25");
    }

    #[test]
    fn test_binding_option() {
        assert_eq!(some_number(|| Some(42)), "The Answer: 42!");
        assert_eq!(some_number(|| Some(13)), "Not interesting... 13");
    }
}
