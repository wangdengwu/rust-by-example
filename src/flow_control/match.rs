mod destructure_slice;
mod destructure_tuple;
mod destructure_enum;
mod destructure_pointers;
mod destructure_structures;
mod guard;
mod binding;

fn match_number(number: i32) {
    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }
}

fn match_boolean(boolean: bool) -> i32 {
    // Match is an expression too
    match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        // TODO ^ Try different values for `number`
        match_number(1);
        match_number(2);
        match_number(13);
        match_number(20);

        assert_eq!(match_boolean(true), 1);
        assert_eq!(match_boolean(false), 0);
    }
}
