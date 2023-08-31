struct Person {
    name: String,
    age: u8,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

mod test {
    use super::*;

    #[test]
    fn test_person_struct() {
        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };
        assert_eq!(peter.name, "Peter");
        assert_eq!(peter.age, 27);
    }

    #[test]
    fn test_unit_struct() {
        let unit = Unit;
        assert_eq!(unit, Unit);
    }

    #[test]
    fn test_pair_struct() {
        let pair = Pair(1, 0.1);
        assert_eq!(pair.0, 1);
        assert_eq!(pair.1, 0.1);
        let Pair(integer, decimal) = pair;
        assert_eq!(integer, 1);
        assert_eq!(decimal, 0.1);
    }

    #[test]
    fn test_point_struct() {
        let point: Point = Point { x: 10.3, y: 0.4 };
        assert_eq!(point.x, 10.3);
        assert_eq!(point.y, 0.4);
    }

    #[test]
    fn test_rectangle_struct() {
        let left_top = Point { x: 10.3, y: 0.4 };
        let bottom_right = Point { x: 5.2, ..left_top };
        println!("{:?}", bottom_right);
        let Point { x: left_edge, y: top_edge } = left_top;
        let rectangle = Rectangle {
            top_left: Point { x: left_edge, y: top_edge },
            bottom_right,
        };
        println!("{:?}", rectangle);
        assert_eq!(rectangle.bottom_right.x, 5.2);
        assert_eq!(rectangle.top_left.y, 0.4);
    }
}