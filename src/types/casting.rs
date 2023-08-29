#![allow(overflowing_literals)]

#[cfg(test)]
mod tests {
    #[test]
    fn test_casting() {
        let decimal = 65.4321_f32;

        // Error! No implicit conversion
        #[cfg(error)]// @formatter:off
        let integer: u8 = decimal;

        // Explicit conversion
        let integer = decimal as u8;
        let character = integer as char;

        // Error! There are limitations in conversion rules.
        // A float cannot be directly converted to a char.
        #[cfg(error)]// @formatter:off
        let character = decimal as char;

        assert_eq!(65.4321, decimal);
        assert_eq!(65, integer);
        assert_eq!('A', character);

        // when casting any value to an unsigned type, T,
        // T::MAX + 1 is added or subtracted until the value
        // fits into the new type

        // 1000 already fits in a u16
        assert_eq!(1000, 1000u16);

        // 1000 - 256 - 256 - 256 = 232
        // Under the hood, the first 8 least significant bits (LSB) are kept,
        // while the rest towards the most significant bit (MSB) get truncated.
        assert_eq!(232, 1000u8);
        // -1 + 256 = 255
        assert_eq!(255, (-1i8) as u8);

        // For positive numbers, this is the same as the modulus
        assert_eq!(232, 1000 % 256);

        // When casting to a signed type, the (bitwise) result is the same as
        // first casting to the corresponding unsigned type. If the most significant
        // bit of that value is 1, then the value is negative.

        // Unless it already fits, of course.
        assert_eq!(128, 128i16);
        // In boundary case 128 value in 8-bit two's complement representation is -128
        assert_eq!(-128, 128i8);

        // repeating the example above
        // 1000 as u8 -> 232
        assert_eq!(232, 1000u8);
        // and the value of 232 in 8-bit two's complement representation is -24
        assert_eq!(-24, 232i8);

        // Since Rust 1.45, the `as` keyword performs a *saturating cast*
        // when casting from float to int. If the floating point value exceeds
        // the upper bound or is less than the lower bound, the returned value
        // will be equal to the bound crossed.

        // 300.0 as u8 is 255
        assert_eq!(255, 300.0_f32 as u8);
        // -100.0 as u8 is 0
        assert_eq!(0, -100.0_f32 as u8);
        // nan as u8 is 0
        assert_eq!(0, f32::NAN as u8);

        // This behavior incurs a small runtime cost and can be avoided
        // with unsafe methods, however the results might overflow and
        // return **unsound values**. Use these methods wisely:
        unsafe {
            // 300.0 as u8 is 44
            assert_eq!(44, 300.0_f32.to_int_unchecked::<u8>());
            // -100.0 as u8 is 0
            assert_eq!(0, (-100.0_f32).to_int_unchecked::<u8>());
            // nan as u8 is 0
            assert_eq!(0, f32::NAN.to_int_unchecked::<u8>());
        }
    }
}