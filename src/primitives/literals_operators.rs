#[cfg(test)]
mod tests {
    #[test]
    fn test_integer_addition() {
        assert_eq!(3, 1u32 + 2);
    }

    #[test]
    fn test_integer_subtraction() {
        // error: this arithmetic operation will overflow
        #[cfg(error)]
        println!("1 - 2 = {}", 1u32 - 2);
        assert_eq!(-1, 1 - 2);
    }

    #[test]
    fn test_scientific_notation() {
        let o = format!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
        assert_eq!("1e4 is 10000, -2.5e-3 is -0.0025", o);
    }

    #[test]
    fn test_bitwise_operations() {
        let o = format!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        assert_eq!("0011 AND 0101 is 0001", o);
        let o = format!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        assert_eq!("0011 OR 0101 is 0111", o);
        let o = format!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        assert_eq!("0011 XOR 0101 is 0110", o);
        let o = format!("1 << 5 is {}", 1u32 << 5);
        assert_eq!("1 << 5 is 32", o);
        let o = format!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
        assert_eq!("0x80 >> 2 is 0x20", o);
    }

    #[test]
    fn test_underscores() {
        let o = format!("One million is written as {}", 1_000_000u32);
        assert_eq!("One million is written as 1000000", o);
    }
}