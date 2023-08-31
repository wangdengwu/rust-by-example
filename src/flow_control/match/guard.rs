#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

#[cfg(test)]
mod tests {
    use super::Temperature;

    #[test]
    fn test_guard_temperature() {
        let temperature = Temperature::Celsius(35);
        // ^ TODO try different values for `temperature`
        let result =
            match temperature {
                Temperature::Celsius(t) if t > 30 => format!("{}C is above 30 Celsius", t),
                // The `if condition` part ^ is a guard
                Temperature::Celsius(t) => format!("{}C is below 30 Celsius", t),

                Temperature::Fahrenheit(t) if t > 86 => format!("{}F is above 86 Fahrenheit", t),
                Temperature::Fahrenheit(t) => format!("{}F is below 86 Fahrenheit", t),
            };
        assert_eq!(result, "35C is above 30 Celsius");
    }

    #[test]
    fn test_guard_unsigned_integer() {
        let number: u8 = 4;
        let result =
            match number {
                i if i == 0 => format!("Zero"),
                i if i > 0 => format!("Greater than zero"),
                _ => unreachable!("Should never happen."),
            };
        assert_eq!(result, "Greater than zero");
    }
}