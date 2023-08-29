#[cfg(test)]
mod test {
    #[test]
    fn test_freezing() {
        let mut _mutable_integer = 7i32;

        {
            // Shadowing by immutable `_mutable_integer`
            let _mutable_integer = _mutable_integer;

            // Error! `_mutable_integer` is frozen in this scope
            #[cfg(error)]
            {
                _mutable_integer = 50;
            }

            // `_mutable_integer` goes out of scope
        }

        // Ok! `_mutable_integer` is not frozen in this scope
        _mutable_integer = 3;

        assert_eq!(3, _mutable_integer);
    }
}