use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

mod testcase_list;
mod formatting;

struct Structure(i32);

impl Display for Structure {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_display() {
        assert_eq!(format!("{}", Structure(3)), "3");
    }

    #[test]
    fn test_min_max_display() {
        let minmax = MinMax(0, 14);
        assert_eq!(format!("{}", minmax), "(0, 14)");
    }

    #[test]
    fn test_min_max_debug() {
        let minmax = MinMax(0, 14);
        assert_eq!(format!("{:?}", minmax), "MinMax(0, 14)");
    }

    #[test]
    fn test_point2d_display() {
        let origin = Point2D { x: 3.3, y: 7.2 };
        assert_eq!(format!("{}", origin), "x: 3.3, y: 7.2");
    }

    #[test]
    fn test_point2d_debug() {
        let origin = Point2D { x: 3.3, y: 7.2 };
        assert_eq!(format!("{:?}", origin), "Point2D { x: 3.3, y: 7.2 }");
    }
}