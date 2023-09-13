struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    // Explicitly requires `A` and `B`.
    fn first(&self) -> i32;
    // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
}

impl Contains<i32, i32> for Container {
    // True if the numbers stored are equal.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_problem() {
        let number_1 = 3;
        let number_2 = 10;

        let container = Container(number_1, number_2);

        assert_eq!(true, container.contains(&number_1, &number_2));
        assert_eq!(3, container.first());
        assert_eq!(10, container.last());

        assert_eq!(7, difference(&container));
    }
}