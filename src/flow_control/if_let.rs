// Our example enum
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

#[cfg(test)]
mod tests {
    use super::Foo;

    #[test]
    fn test_if_let() {
        // All have type `Option<i32>`
        let number = Some(7);
        let letter: Option<i32> = None;
        let emoticon: Option<i32> = None;

        // The `if let` construct reads: "if `let` destructures `number` into
        // `Some(i)`, evaluate the block (`{}`).
        if let Some(i) = number {
            assert_eq!(i, 7);
        }

        // If you need to specify a failure, use an else:
        if let Some(i) = letter {
            unreachable!("{:?}", i);
        } else {
            // Destructure failed. Change to the failure case.
            println!("Didn't match a number. Let's go with a letter!");
            assert!(true);
        }

        // Provide an altered failing condition.
        let i_like_letters = false;

        if let Some(i) = emoticon {
            unreachable!("{:?}", i);
            // Destructure failed. Evaluate an `else if` condition to see if the
            // alternate failure branch should be taken:
        } else if i_like_letters {
            unreachable!("{}", ("Didn't match a number. Let's go with a letter!"));
        } else {
            // The condition evaluated false. This branch is the default:
            println!("I don't like letters. Let's go with an emoticon :)!");
            assert!(true);
        }
    }

    #[test]
    fn test_if_let_enums() {
        // Create example variables
        let a = Foo::Bar;
        let b = Foo::Baz;
        let c = Foo::Qux(100);

        // Variable a matches Foo::Bar
        if let Foo::Bar = a {
            assert!(true);
        }

        // Variable b does not match Foo::Bar
        // So this will print nothing
        if let Foo::Baz = b {
            assert!(true);
        }

        // Variable c matches Foo::Qux which has a value
        // Similar to Some() in the previous example
        if let Foo::Qux(value) = c {
            assert_eq!(value, 100);
        }

        // Binding also works with `if let`
        if let Foo::Qux(_value @ 100) = c {
            assert!(true);
        }
    }
}