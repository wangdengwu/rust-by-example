#[cfg(test)]
mod tests {
    use std::mem::size_of_val;

    #[test]
    fn test_literals() {
        // Suffixed literals, their types are known at initialization
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;

        // Unsuffixed literals, their types depend on how they are used
        let i = 1;
        let f = 1.0;

        let b = true;
        let c = 'c';
        let s = '🦀';

        // `size_of_val` returns the size of a variable in bytes
        assert_eq!(1, size_of_val(&x));
        assert_eq!(4, size_of_val(&y));
        assert_eq!(4, size_of_val(&z));
        assert_eq!(4, size_of_val(&i));
        assert_eq!(8, size_of_val(&f));
        assert_eq!(1, size_of_val(&b));
        assert_eq!(4, size_of_val(&c));
        assert_eq!(4, size_of_val(&s));
    }
}