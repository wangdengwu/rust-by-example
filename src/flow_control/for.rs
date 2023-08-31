#[cfg(test)]
mod tests {
    use crate::flow_control::test_loop;

    #[test]
    fn test_for_range_exclusive() {
        let mut fizzbuzz = 0;
        let mut fizz = 0;
        let mut buzz = 0;
        let mut counter = 0;
        // `n` will take the values: 1, 2, ..., 100 in each iteration
        for n in 1..101 {
            test_loop(&mut fizzbuzz, &mut fizz, &mut buzz, &mut counter, n);
        }
        assert_eq!(6, fizzbuzz);
        assert_eq!(27, fizz);
        assert_eq!(14, buzz);
        assert_eq!(53, counter);
    }

    #[test]
    fn test_for_range_inclusive() {
        let mut fizzbuzz = 0;
        let mut fizz = 0;
        let mut buzz = 0;
        let mut counter = 0;
        // `n` will take the values: 1, 2, ..., 100 in each iteration
        for n in 1..=100 {
            test_loop(&mut fizzbuzz, &mut fizz, &mut buzz, &mut counter, n);
        }
        assert_eq!(6, fizzbuzz);
        assert_eq!(27, fizz);
        assert_eq!(14, buzz);
        assert_eq!(53, counter);
    }

    #[test]
    fn test_for_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];
        let mut match_name_count = 0;
        let mut match_other_count = 0;
        for name in names.iter() {
            match name {
                &"Ferris" => match_name_count += 1,
                _ => match_other_count += 1,
            }
        }
        println!("names: {:?}", names);
        assert_eq!(1, match_name_count);
        assert_eq!(2, match_other_count);
    }

    #[test]
    fn test_for_iter_mut() {
        let mut names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
        println!("names: {:?}", names);
        assert_eq!(vec!["Hello", "Hello", "There is a rustacean among us!"], names);
    }

    #[test]
    fn test_for_into_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];
        let mut match_name_count = 0;
        let mut match_other_count = 0;
        for name in names.into_iter() {
            match name {
                "Ferris" => match_name_count += 1,
                _ => match_other_count += 1,
            }
        }

        #[cfg(error)]
        println!("names: {:?}", names);

        assert_eq!(1, match_name_count);
        assert_eq!(2, match_other_count);
    }
}