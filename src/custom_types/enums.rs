enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) -> String {
    match event {
        WebEvent::PageLoad => format!("page loaded"),
        WebEvent::PageUnload => format!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => format!("pressed '{}'.", c),
        WebEvent::Paste(s) => format!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            format!("clicked at x={}, y={}.", x, y)
        }
    }
}

mod type_aliases {
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    pub(crate) type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    impl Operations {
        pub(crate) fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Number {
    Zero,
    One,
    Two,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::type_aliases::Operations;

    #[test]
    fn test_inspect() {
        let pressed = WebEvent::KeyPress('x');
        let pasted = WebEvent::Paste("my text".to_owned());
        let click = WebEvent::Click { x: 20, y: 80 };
        let load = WebEvent::PageLoad;
        let unload = WebEvent::PageUnload;
        assert_eq!(inspect(pressed), "pressed 'x'.");
        assert_eq!(inspect(pasted), "pasted \"my text\".");
        assert_eq!(inspect(click), "clicked at x=20, y=80.");
        assert_eq!(inspect(load), "page loaded");
        assert_eq!(inspect(unload), "page unloaded");
    }

    #[test]
    fn test_type_aliases() {
        let add = Operations::Add;
        assert_eq!(add, Operations::Add);
        let subtract = Operations::Subtract;
        assert_eq!(subtract, Operations::Subtract);
        assert_eq!(add.run(1, 2), 3);
        assert_eq!(subtract.run(3, 2), 1);
    }

    #[test]
    fn test_number_enum() {
        let zero = Number::Zero;
        let one = Number::One;
        let two = Number::Two;
        assert_eq!(Number::Zero as i32, 0);
        assert_eq!(Number::One as i32, 1);
        assert_eq!(Number::Two as i32, 2);
        assert_eq!(zero, Number::Zero);
        assert_eq!(one, Number::One);
        assert_eq!(two, Number::Two);
    }

    #[test]
    fn test_color_enum() {
        let red = Color::Red;
        let green = Color::Green;
        let blue = Color::Blue;
        assert_eq!(red, Color::Red);
        assert_eq!(red as u32, 0xff0000);
        assert_eq!(green, Color::Green);
        assert_eq!(green as u32, 0x00ff00);
        assert_eq!(blue, Color::Blue);
        assert_eq!(blue as u32, 0x0000ff);
    }
}