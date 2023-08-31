#[cfg(test)]
mod tests {
    #[test]
    fn test_loop() {
        let mut count = 0u32;

        // Infinite loop
        loop {
            count += 1;

            if count == 3 {
                // Skip the rest of this iteration
                continue;
            }

            if count == 5 {
                // Exit this loop
                break;
            }
        }
        assert_eq!(count, 5);
    }

    #[test]
    #[allow(unused_labels, unreachable_code)]
    fn test_nesting_labels() {
        'outer: loop {
            'inner: loop {
                // This would break only the inner loop
                //break;

                // This breaks the outer loop
                break 'outer;
            }
            assert!(false);
        }

        assert!(true);
    }

    #[test]
    fn test_returning_from_loops() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);
    }
}