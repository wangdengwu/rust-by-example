struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl() {
        let x = Val { val: 3.0 };
        let y = GenVal { gen_val: 3i32 };
        assert_eq!(3.0, *x.value());
        assert_eq!(3i32, *y.value());
    }
}