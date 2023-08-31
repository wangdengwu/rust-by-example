struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_origin() {
        let p = Point::origin();
        assert_eq!(0.0, p.x);
        assert_eq!(0.0, p.y);
    }

    #[test]
    fn test_point_new() {
        let p = Point::new(3.3, 4.4);
        assert_eq!(3.3, p.x);
        assert_eq!(4.4, p.y);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle {
            p1: Point::origin(),
            p2: Point::new(3.0, 4.0),
        };
        // Error! `rect` is immutable, but this method requires a mutable object
        #[cfg(error)]
        rect.translate(4, 4);
        assert_eq!(12.0, rect.area());
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle {
            p1: Point::origin(),
            p2: Point::new(3.0, 4.0),
        };
        assert_eq!(14.0, rect.perimeter());
    }

    #[test]
    fn test_rectangle_translate() {
        let mut rect = Rectangle {
            p1: Point::origin(),
            p2: Point::new(3.0, 4.0),
        };
        rect.translate(3.0, 3.0);
        assert_eq!(3.0, rect.p1.x);
        assert_eq!(3.0, rect.p1.y);
        assert_eq!(6.0, rect.p2.x);
        assert_eq!(7.0, rect.p2.y);
    }

    #[test]
    fn test_pair_destroy() {
        let pair = Pair(Box::new(1), Box::new(2));

        pair.destroy();

        // Error! Previous `destroy` call "consumed" `pair`
        #[cfg(error)]
        pair.destroy();
    }
}