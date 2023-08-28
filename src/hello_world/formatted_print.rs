fn format_by_default() -> String {
    format!("{} days", 31)
}

fn format_by_index() -> String {
    format!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob")
}

fn format_by_named() -> String {
    format!("{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over")
}

fn format_by_binary_type() -> String {
    format!("Base 2 (binary):       {:b}", 69420) // 10000111100101100
}

fn format_by_octal_type() -> String {
    format!("Base 8 (octal):        {:o}", 69420)
}

fn format_by_hexadecimal_type() -> String {
    format!("Base 16 (hexadecimal):  {:x}", 69420)
}

fn format_by_right_justify_default() -> String {
    format!("{number:>5}", number = 1)
}

fn format_by_right_justify_0() -> String {
    format!("{number:0>5}", number = 1)
}

fn format_by_left_adjust_0() -> String {
    format!("{number:0<5}", number = 1)
}

fn format_by_right_justify_width() -> String {
    format!("{number:0>width$}", number = 1, width = 5)
}

fn format_by_pi() -> String {
    format!("{:.3}", std::f64::consts::PI)
}

#[derive(Debug)]
struct Structure(i32);

fn format_by_struct() -> String {
    format!("This struct `{:?}` won't print...", Structure(3))
}

fn format_by_local_variable() -> String {
    let number: f64 = 1.0;
    let width: usize = 5;
    format!("{number:0>width$}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_by_default() {
        assert_eq!(format_by_default(), "31 days");
    }

    #[test]
    fn test_format_by_index() {
        assert_eq!(format_by_index(), "Alice, this is Bob. Bob, this is Alice");
    }

    #[test]
    fn test_format_by_named() {
        assert_eq!(format_by_named(), "the quick brown fox jumps over the lazy dog");
    }

    #[test]
    fn test_format_by_binary_type() {
        assert_eq!(format_by_binary_type(), "Base 2 (binary):       10000111100101100");
    }

    #[test]
    fn test_format_by_octal_type() {
        assert_eq!(format_by_octal_type(), "Base 8 (octal):        207454");
    }

    #[test]
    fn test_format_by_hexadecimal_type() {
        assert_eq!(format_by_hexadecimal_type(), "Base 16 (hexadecimal):  10f2c");
    }

    #[test]
    fn test_format_by_right_justify_default() {
        assert_eq!(format_by_right_justify_default(), "    1");
    }

    #[test]
    fn test_format_by_right_justify_0() {
        assert_eq!(format_by_right_justify_0(), "00001");
    }

    #[test]
    fn test_format_by_left_adjust_0() {
        assert_eq!(format_by_left_adjust_0(), "10000");
    }

    #[test]
    fn test_format_by_right_justify_width() {
        assert_eq!(format_by_right_justify_width(), "00001");
    }

    #[test]
    fn test_format_by_struct() {
        assert_eq!(format_by_struct(), "This struct `Structure(3)` won't print...");
    }

    #[test]
    fn test_format_by_local_variable() {
        assert_eq!(format_by_local_variable(), "00001");
    }

    #[test]
    fn test_format_by_pi() {
        assert_eq!(format_by_pi(), "3.142");
    }
}
