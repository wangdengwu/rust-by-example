#[cfg(test)]
mod tests {
    #[test]
    fn test_scope() {
        let long_lived_binding = 1;

        // This is a block, and has a smaller scope than the main function
        {
            // This binding only exists in this block
            let short_lived_binding = 2;

            assert_eq!(2, short_lived_binding);
        }
        // End of the block

        // Error! `short_lived_binding` doesn't exist in this scope
        #[cfg(error)]
        println!("outer short: {}", short_lived_binding);

        assert_eq!(1, long_lived_binding);
    }

    #[test]
    fn test_shadowing() {
        let shadowed_binding = 1;
        {
            println!("before being shadowed: {}", shadowed_binding);

            // This binding *shadows* the outer one
            let shadowed_binding = "abc";

            assert_eq!("abc", shadowed_binding);
        }
        assert_eq!(1, shadowed_binding);

        // This binding *shadows* the previous binding
        let shadowed_binding = 2u8;
        assert_eq!(2, shadowed_binding);
    }
}