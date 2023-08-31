fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fn_mut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fn_once() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_parameters() {
        let fn_plain = create_fn();
        let mut fn_mut = create_fn_mut();
        let fn_once = create_fn_once();

        fn_plain();
        fn_mut();
        fn_once();
    }
}