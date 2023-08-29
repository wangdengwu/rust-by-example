#[cfg(test)]
mod tests {
    #[test]
    fn test_declare_first() {
        // Declare a variable binding
        let a_binding;

        {
            let x = 2;

            // Initialize the binding
            a_binding = x * x;
        }

        assert_eq!(4, a_binding);

        let another_binding;
        
        // Error! Use of uninitialized binding
        #[cfg(error)]
        assert_eq!(0, another_binding);

        another_binding = 1;

        assert_eq!(1, another_binding);
    }
}