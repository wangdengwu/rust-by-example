use std::fmt::{Display, Formatter, Result};

struct Circle {
    radius: i32,
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_display() {
        let circle = Circle { radius: 3 };
        assert_eq!("Circle of radius 3", format!("{}", circle));
    }

    #[test]
    fn test_parsing_string() {
        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();

        let sum = parsed + turbo_parsed;
        assert_eq!(15, sum);
    }
}