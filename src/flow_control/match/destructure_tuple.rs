#[cfg(test)]
mod tests {
    #[test]
    fn test_tuples() {
        let triple = (0, -2, 2);
        // TODO ^ Try different values for `triple`

        println!("Tell me about {:?}", triple);
        // Match can be used to destructure a tuple
        let index = match triple {
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
        };
        assert_eq!(1, index);
    }
}