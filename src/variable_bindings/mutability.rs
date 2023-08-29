#[cfg(test)]
mod tests {
    #[test]
    fn test_mutability() {
        let _immutable_binding = 1;
        let mut mutable_binding = 1;
        assert_eq!(1, mutable_binding);
        mutable_binding += 1;
        assert_eq!(2, mutable_binding);
    }

    #[test]
    #[cfg(error)]
    fn test_compile_error() {
        let _immutable_binding = 1;
        _immutable_binding = 2;
    }
}