use std::str::FromStr;

fn get_count_item(s: &str) -> (u32, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u32::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_count_item() {
        assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    }

    #[test]
    #[should_panic]
    fn test_get_count_item_split() {
        get_count_item("chairs");
    }

    #[test]
    #[should_panic]
    fn test_get_count_item_parse() {
        get_count_item("a chairs");
    }
}