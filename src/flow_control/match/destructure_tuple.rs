fn match_tuple(triple: (i32, i32, i32)) -> i32 {
    println!("Tell me about {:?}", triple);
    match triple {
        // Destructure the second and third elements
        (0, y, z) => {
            println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z);
            1
        }
        (1, ..) => {
            println!("First is `1` and the rest doesn't matter");
            2
        }
        (.., 2) => {
            println!("last is `2` and the rest doesn't matter");
            3
        }
        (3, .., 4) => {
            println!("First is `3`, last is `4`, and the rest doesn't matter");
            4
        }
        // `..` can be used to ignore the rest of the tuple
        _ => {
            println!("It doesn't matter what they are");
            5
        }
        // `_` means don't bind the value to a variable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuples() {
        // Match can be used to destructure a tuple
        // TODO ^ Try different values for `triple`
        assert_eq!(1, match_tuple((0, -2, 2)));
        assert_eq!(2, match_tuple((1, -2, 2)));
        assert_eq!(3, match_tuple((3, -2, 2)));
        assert_eq!(4, match_tuple((3, -2, 4)));
        assert_eq!(5, match_tuple((6, 12, 12)));
    }
}