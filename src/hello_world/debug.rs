#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn format_debug_by_default() -> String {
    format!("{:?} months in a year.", 12)
}

fn format_debug_by_index() -> String {
    format!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    )
}

fn format_debug_by_struct() -> String {
    format!("Now {:?} will print!", Structure(3))
}

fn format_debug_by_struct_deep() -> String {
    format!("Now {:?} will print!", Deep(Structure(7)))
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn format_debug_by_pretty_print() -> String {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    peter.name;
    peter.age;
    format!("{:#?}", peter)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_debug_by_default() {
        assert_eq!(format_debug_by_default(), "12 months in a year.");
    }

    #[test]
    fn test_format_debug_by_index() {
        let result_str = r#""Christian" "Slater" is the "actor's" name."#;
        assert_eq!(format_debug_by_index(), result_str);
    }

    #[test]
    fn test_format_debug_by_struct() {
        assert_eq!(format_debug_by_struct(), "Now Structure(3) will print!");
    }

    #[test]
    fn test_format_debug_by_struct_deep() {
        assert_eq!(format_debug_by_struct_deep(), "Now Deep(Structure(7)) will print!");
    }

    #[test]
    fn test_format_debug_by_pretty_print() {
        #[rustfmt::skip]//@formatter:off
        let result_str =
r#"Person {
    name: "Peter",
    age: 27,
}"#;//@formatter:on
        assert_eq!(format_debug_by_pretty_print(), result_str);
    }
}
