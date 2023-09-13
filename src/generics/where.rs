use std::fmt::{Debug, Display};

trait TraitB {}

trait TraitC {}

trait TraitE {}

trait TraitF {}

trait MyTrait<A, D> {
    fn print_in_option(self) -> String;
}

struct YourType;

impl Display for YourType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "YourType")
    }
}

impl TraitB for YourType {}

impl TraitC for YourType {}

impl TraitE for YourType {}

impl TraitF for YourType {}

// Expressing bounds with a `where` clause
impl<A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {
    fn print_in_option(self) -> String {
        format!("{}", self)
    }
}

trait PrintInOption {
    fn print_in_option(self) -> String;
}

// Because we would otherwise have to express this as `T: Debug` or
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) -> String {
        format!("{:?}", Some(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_where() {
        let vec = vec![1, 2, 3];

        assert_eq!("Some([1, 2, 3])", vec.print_in_option());

        let your_type = YourType;
        assert_eq!("YourType", <YourType as MyTrait<YourType, YourType>>::print_in_option(your_type));
    }
}