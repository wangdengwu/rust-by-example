// `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aliasing() {
        // `NanoSecond` = `Inch` = `U64` = `u64`.
        let nanoseconds: NanoSecond = 5 as U64;
        let inches: Inch = 2 as U64;

        // Note that type aliases *don't* provide any extra type safety, because
        // aliases are *not* new types
        assert_eq!(5, nanoseconds);
        assert_eq!(2, inches);
        assert_eq!(7, nanoseconds + inches);
    }
}