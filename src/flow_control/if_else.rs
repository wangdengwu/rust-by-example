fn if_else_str(n: i32) -> String {
    if n < 0 {
        format!("{} is negative", n)
    } else if n > 0 {
        format!("{} is positive", n)
    } else {
        format!("{} is zero", n)
    }
}

fn if_else_int(n: i32) -> i32 {
    if n < 10 && n > -10 {
        // This expression returns an `i32`.
        10 * n
    } else {
        // This expression must return an `i32` as well.
        n / 2
        // TODO ^ Try suppressing this expression with a semicolon.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_else() {
        assert_eq!(if_else_str(5), "5 is positive");
        assert_eq!(if_else_str(0), "0 is zero");
        assert_eq!(if_else_str(-5), "-5 is negative");

        assert_eq!(if_else_int(5), 50);
        assert_eq!(if_else_int(20), 10);
    }
}