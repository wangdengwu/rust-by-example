mod formatted_print;
mod debug;
mod display;

/// returns "Hello, world!"
fn hello_world() -> String {
    // this is a comment
    "Hello, world!".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!("Hello, world!", hello_world());
    }
}