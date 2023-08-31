#[allow(dead_code)]
fn foo() -> ! {
    panic!("This call never returns.");
}

fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        // Notice that the return type of this match expression must be u32
        // because of the type of the "addition" variable.
        let addition: u32 = match i % 2 == 1 {
            // The "i" variable is of type u32, which is perfectly fine.
            true => i,
            // On the other hand, the "continue" expression does not return
            // u32, but it is still fine, because it never returns and therefore
            // does not violate the type requirements of the match expression.
            false => continue,
        };
        acc += addition;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::sum_odd_numbers;

    #[test]
    fn test_diverging() {
        assert_eq!(16, sum_odd_numbers(9));
    }
}