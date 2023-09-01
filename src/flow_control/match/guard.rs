enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn match_guard(temperature: Temperature) -> String {
    match temperature {
        Temperature::Celsius(t) if t > 30 => format!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celsius(t) => format!("{}C is below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => format!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => format!("{}F is below 86 Fahrenheit", t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_temperature() {
        // ^ TODO try different values for `temperature`
        assert_eq!(match_guard(Temperature::Celsius(35)), "35C is above 30 Celsius");
        assert_eq!(match_guard(Temperature::Celsius(30)), "30C is below 30 Celsius");
        assert_eq!(match_guard(Temperature::Fahrenheit(30)), "30F is below 86 Fahrenheit");
        assert_eq!(match_guard(Temperature::Fahrenheit(90)), "90F is above 86 Fahrenheit");
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