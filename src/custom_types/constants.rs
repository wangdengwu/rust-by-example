static mut LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_big() {
        let n = 16;
        unsafe { assert_eq!("Rust", LANGUAGE); }
        assert_eq!(10, THRESHOLD);
        assert_eq!("big", if is_big(n) { "big" } else { "small" });

        unsafe { LANGUAGE = "rust"; }
        unsafe { assert_eq!("rust", LANGUAGE); }
    }

    #[test]
    #[cfg(error)]
    fn test_modify_const() {
        // Error! Cannot modify a `const`.
        THRESHOLD = 5;
    }
}