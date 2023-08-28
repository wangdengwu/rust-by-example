mod literals_operators;
mod tuples;
mod array_slices;

#[cfg(test)]
mod tests {
    #[test]
    fn test_signed_integers() {
        let i8 = -128i8;
        assert_eq!(-128, i8);
        let i16 = -32768i16;
        assert_eq!(-32768, i16);
        let i32 = -2147483648;
        assert_eq!(-2147483648, i32);
        let i64 = -9223372036854775808i64;
        assert_eq!(-9223372036854775808, i64);
        let isize = -9223372036854775808isize;
        assert_eq!(-9223372036854775808, isize);
        let i128 = -170141183460469231731687303715884105728i128;
        assert_eq!(-170141183460469231731687303715884105728, i128);
    }

    #[test]
    fn test_unsigned_integers() {
        let u8 = 255u8;
        assert_eq!(255, u8);
        let u16 = 65535u16;
        assert_eq!(65535, u16);
        let u32: u32 = 4294967295;
        assert_eq!(4294967295, u32);
        let u64 = 18446744073709551615u64;
        assert_eq!(18446744073709551615, u64);
        let usize = 18446744073709551615usize;
        assert_eq!(18446744073709551615, usize);
        let u128 = 340282366920938463463374607431768211455u128;
        assert_eq!(340282366920938463463374607431768211455, u128);
    }

    #[test]
    fn test_floats() {
        let f32 = 4.1415921f32;
        assert_eq!(4.1415921, f32);
        let f64 = 1.618033f64;
        assert_eq!(1.618033, f64);
    }

    #[test]
    fn test_char() {
        let c = 'a';
        assert_eq!('a', c);
        let c: char = '😀';
        assert_eq!('😀', c);
    }

    #[test]
    fn test_bool() {
        let b = true;
        assert_eq!(true, b);
        let b: bool = false;
        assert_eq!(false, b);
    }

    #[test]
    fn test_unit() {
        let u: () = ();
        assert_eq!((), u);
    }

    #[test]
    fn test_tuple() {
        let t: (i32, f64, u8) = (500, 6.4, 1);
        assert_eq!((500, 6.4, 1), t);
    }

    #[test]
    fn test_array() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!([1, 2, 3, 4, 5], a);
    }

    #[test]
    fn test_slice() {
        let s = [1, 2, 3, 4, 5];
        assert_eq!([2, 3, 4], s[1..4]);
    }
}