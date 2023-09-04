// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

fn function() -> String {
    format!("called `function()`")
}

mod deeply {
    pub mod nested {
        pub fn function() -> String {
            format!("called `deeply::nested::function()`")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use() {
        assert_eq!("called `deeply::nested::function()`", other_function());
        {
            // This is equivalent to `use deeply::nested::function as function`.
            // This `function()` will shadow the outer one.
            use crate::modules::r#use::deeply::nested::function;

            // `use` bindings have a local scope. In this case, the
            // shadowing of `function()` is only in this block.
            assert_eq!("called `deeply::nested::function()`", function());
        }

        assert_eq!("called `function()`", function());
    }
}