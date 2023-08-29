mod mutability;
mod scope_shadowing;
mod declare_first;
mod freezing;

#[cfg(test)]
mod tests {
    #[test]
    fn test_variable_bindings() {
        let an_integer = 1;
        let a_boolean = true;
        let unit = ();
        let copied_integer = an_integer;

        assert_eq!(1, copied_integer);
        assert!(a_boolean);
        assert_eq!((), unit);

        let _unused_variable = 3u32;

        #[allow(unused_variables)] // @formatter:off
        let noisy_unused_variable = 2u32;
    }
}