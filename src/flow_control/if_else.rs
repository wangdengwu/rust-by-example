#[cfg(test)]
mod tests {
    #[test]
    fn test_if_else() {
        let n = 5;

        let result =
            if n < 0 {
                format!("{} is negative", n)
            } else if n > 0 {
                format!("{} is positive", n)
            } else {
                format!("{} is zero", n)
            };

        let big_n =
            if n < 10 && n > -10 {
                // This expression returns an `i32`.
                10 * n
            } else {
                // This expression must return an `i32` as well.
                n / 2
                // TODO ^ Try suppressing this expression with a semicolon.
            };
        //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

        assert_eq!(result, "5 is positive");
        assert_eq!(big_n, 50);
    }
}