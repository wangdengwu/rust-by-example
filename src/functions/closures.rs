mod capture;
mod input_parameters;
mod anonymity;
mod input_functions;
mod output_parameters;
mod iter_any;
mod iter_find;

#[cfg(test)]
mod tests {
    #[test]
    fn test_closure() {
        let outer_var = 42;

        // A regular function can't refer to variables in the enclosing environment
        #[cfg(error)]
        fn function(i: i32) -> i32 { i + outer_var }
        // The compiler suggests that we define a closure instead.

        // Closures are anonymous, here we are binding them to references
        // Annotation is identical to function annotation but is optional
        // as are the `{}` wrapping the body. These nameless functions
        // are assigned to appropriately named variables.
        let closure_annotated = |i: i32| -> i32 { i + outer_var };
        let closure_inferred = |i| i + outer_var;

        // Call the closures.
        assert_eq!(43, closure_annotated(1));
        assert_eq!(44, closure_inferred(2));
        // Once closure's type has been inferred, it cannot be inferred again with another type.
        #[cfg(error)]
        assert_eq!(45, closure_inferred(3i64));

        // A closure taking no arguments which returns an `i32`.
        // The return type is inferred.
        let one = || 1;
        assert_eq!(1, one());
    }
}