#[cfg(test)]
mod tests {
    use crate::flow_control::test_loop;

    #[test]
    fn test_while() {
        // A counter variable
        let mut n = 1;
        let mut fizzbuzz = 0;
        let mut fizz = 0;
        let mut buzz = 0;
        let mut counter = 0;
        // Loop while `n` is less than 101
        while n < 101 {
            test_loop(&mut fizzbuzz, &mut fizz, &mut buzz, &mut counter, n);
            // Increment counter
            n += 1;
        }
        assert_eq!(6, fizzbuzz);
        assert_eq!(27, fizz);
        assert_eq!(14, buzz);
        assert_eq!(53, counter);
        assert_eq!(101, n);
    }
}