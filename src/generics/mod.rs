mod gen_fn;
mod r#impl;
mod gen_trait;
mod bounds;
mod multi_bounds;
mod r#where;
mod new_types;
mod assoc_items;
mod phantom;

// A concrete type `A`.
#[derive(Debug)]
#[derive(PartialEq)]
struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined as above.
struct Single(A);
//            ^ Here is `Single`s first use of the type `A`.

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top.
struct SingleGen<T>(T);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generics() {
        // `Single` is concrete and explicitly takes `A`.
        let s = Single(A);
        assert_eq!(A, s.0);

        // Create a variable `_char` of type `SingleGen<char>`
        // and give it the value `SingleGen('a')`.
        // Here, `SingleGen` has a type parameter explicitly specified.
        let c: SingleGen<char> = SingleGen('a');
        assert_eq!('a', c.0);

        // `SingleGen` can also have a type parameter implicitly specified:
        let t = SingleGen(A); // Uses `A` defined at the top.
        assert_eq!(A, t.0);

        let i32 = SingleGen(6); // Uses `i32`.
        assert_eq!(6, i32.0);

        let char = SingleGen('a'); // Uses `char`.
        assert_eq!('a', char.0);
    }
}