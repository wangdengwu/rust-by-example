#[derive(Debug)]
#[derive(PartialEq)]
struct A;

// Concrete type `A`.
#[derive(Debug)]
#[derive(PartialEq)]
struct S(A);

// Concrete type `S`.
#[derive(Debug)]
#[derive(PartialEq)]
struct SGen<T>(T); // Generic type `SGen`.

// The following functions all take ownership of the variable passed into
// them and immediately go out of scope, freeing the variable.

// Define a function `reg_fn` that takes an argument `_s` of type `S`.
// This has no `<T>` so this is not a generic function.
fn reg_fn(s: S) -> S {
    s
}

// Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
// It has been explicitly given the type parameter `A`, but because `A` has not
// been specified as a generic type parameter for `gen_spec_t`, it is not generic.
fn gen_spec_t(s: SGen<A>) -> SGen<A> {
    s
}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(s: SGen<i32>) -> SGen<i32> {
    s
}

// Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic<T>(s: SGen<T>) -> SGen<T> {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_fn() {
        // Using the non-generic functions
        assert_eq!(S(A), reg_fn(S(A))); // Concrete type.

        assert_eq!(SGen(A), gen_spec_t(SGen(A)));   // Implicitly specified type parameter `A`.


        assert_eq!(SGen(6), gen_spec_i32(SGen(6))); // Implicitly specified type parameter `i32`.

        // Explicitly specified type parameter `char` to `generic()`.
        assert_eq!(SGen('a'), generic::<char>(SGen('a')));

        // Implicitly specified type parameter `char` to `generic()`.
        assert_eq!(SGen('c'), generic(SGen('c')));
    }
}