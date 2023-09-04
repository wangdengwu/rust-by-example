mod my_mod {
    // Items in modules default to private visibility.
    fn private_function() -> String {
        format!("called `my_mod::private_function()`")
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() -> String {
        format!("called `my_mod::function()`")
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() -> String {
        format!("called `my_mod::indirect_access()`, that \n> {}", private_function())
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() -> String {
            format!("called `my_mod::nested::function()`")
        }

        fn private_function() -> String {
            format!("called `my_mod::nested::private_function()`")
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::modules::visibility::my_mod) fn public_function_in_my_mod() -> String {
            format!("called `my_mod::nested::public_function_in_my_mod()`, that\n> {}", public_function_in_nested())
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() -> String {
            format!("called `my_mod::nested::public_function_in_nested()`")
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() -> String {
            format!("called `my_mod::nested::public_function_in_super_mod()`")
        }

        #[cfg(test)]
        mod tests {
            use super::private_function;

            #[test]
            fn test_nested() {
                assert_eq!("called `my_mod::nested::private_function()`", private_function());
            }
        }
    }

    pub fn call_public_function_in_my_mod() -> String {
        format!("called `my_mod::call_public_function_in_my_mod()`, that\n> {} > {}", nested::public_function_in_my_mod(), nested::public_function_in_super_mod())
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() -> String {
        format!("called `my_mod::public_function_in_crate()`")
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        pub fn function() -> String {
            format!("called `my_mod::private_nested::function()`")
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        pub(crate) fn restricted_function() -> String {
            format!("called `my_mod::private_nested::restricted_function()`")
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::modules::visibility::function;
        use crate::modules::visibility::my_mod;

        #[test]
        fn test_visibility() {
            assert_eq!("called `function()`", function());
            assert_eq!("called `my_mod::function()`", my_mod::function());
            assert_eq!("called `my_mod::indirect_access()`, that \n> called `my_mod::private_function()`", my_mod::indirect_access());
            assert_eq!("called `my_mod::nested::function()`", my_mod::nested::function());
            assert_eq!("called `my_mod::nested::public_function_in_my_mod()`, that\n> called `my_mod::nested::public_function_in_nested()`", my_mod::nested::public_function_in_my_mod());
            assert_eq!("called `my_mod::nested::public_function_in_super_mod()`", my_mod::nested::public_function_in_super_mod());
            assert_eq!("called `my_mod::call_public_function_in_my_mod()`, that\n> called `my_mod::nested::public_function_in_my_mod()`, that\n> called `my_mod::nested::public_function_in_nested()` > called `my_mod::nested::public_function_in_super_mod()`", my_mod::call_public_function_in_my_mod());
            assert_eq!("called `my_mod::public_function_in_crate()`", my_mod::public_function_in_crate());
            assert_eq!("called `my_mod::private_nested::function()`", my_mod::private_nested::function());
            assert_eq!("called `my_mod::private_nested::restricted_function()`", my_mod::private_nested::restricted_function());
        }
    }
}

fn function() -> String {
    format!("called `function()`")
}

