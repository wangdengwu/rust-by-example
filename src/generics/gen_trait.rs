use std::any::{Any, TypeId};

// Non-copyable types.
struct Empty;

struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    fn double_drop(self, t: &T) -> bool;
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`.
impl<T: 'static, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both.
    fn double_drop(self, t: &T) -> bool {
        TypeId::of::<Null>() == t.type_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_trait() {
        let empty = Empty;
        let null = Null;
        // Deallocate `empty` and `null`.
        assert!(empty.double_drop(&null));

        let empty = Empty;
        let null = Null;
        assert!(!null.double_drop(&empty));

        // empty;
        // null;
        // ^ TODO: Try uncommenting these lines.
    }
}