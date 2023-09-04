mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        #[allow(dead_code)]
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_visibility() {
        let open_box = my::OpenBox {
            contents: "public information",
        };
        assert_eq!("public information", open_box.contents);

        // Public structs with private fields cannot be constructed using field names.
        // Error! `ClosedBox` has private fields
        #[cfg(error)] // @formatter:off
        let closed_box = my::ClosedBox { contents: "classified information" };
        // @formatter:on

        // However, structs with private fields can be created using
        // public constructors
        let _closed_box = my::ClosedBox::new("classified information");

        // and the private fields of a public struct cannot be accessed.
        // Error! The `contents` field is private
        #[cfg(error)] // @formatter:off
        println!("The closed box contains: {}", _closed_box.contents);
    }
}