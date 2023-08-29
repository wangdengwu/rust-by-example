pub(self) mod into {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl Into<Number> for i32 {
        fn into(self) -> Number {
            Number { value: self }
        }
    }

    #[cfg(test)]
    mod tests {
        mod into {
            use crate::conversion::from_into::into::Number;

            #[test]
            fn test_into() {
                let num: Number = 30.into();
                assert_eq!(30, num.value);
            }
        }
    }
}

pub(self) mod from {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    #[cfg(test)]
    mod tests {
        mod from {
            use crate::conversion::from_into::from::Number;

            #[test]
            fn test_from() {
                let num = Number::from(30);
                assert_eq!(30, num.value);
            }
        }
    }
}