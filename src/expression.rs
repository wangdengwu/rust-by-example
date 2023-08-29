#[cfg(test)]
mod tests {
    #[test]
    #[allow(warnings)]
    fn test_expression() {
        // variable binding
        let x = 5;

        // expression;
        #[allow(unused)]
        x;
        x + 1;
        15;

        let x = 5u32;

        let y = {
            let x_squared = x * x;
            let x_cube = x_squared * x;

            // This expression will be assigned to `y`
            x_cube + x_squared + x
        };

        let z = {
            // The semicolon suppresses this expression and `()` is assigned to `z`
            2 * x;
        };

        assert_eq!(5, x);
        assert_eq!(5 * 5 * 5 + 5 * 5 + 5, y);
        assert_eq!((), z);
    }
}