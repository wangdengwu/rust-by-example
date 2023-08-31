#[cfg(test)]
mod tests {
    #[test]
    fn test_capture() {
        let color = String::from("green");

        // A closure to print `color` which immediately borrows (`&`) `color` and
        // stores the borrow and closure in the `print` variable. It will remain
        // borrowed until `print` is used the last time.
        //
        // `println!` only requires arguments by immutable reference so it doesn't
        // impose anything more restrictive.
        let print = || format!("`color`: {}", color);
        // Call the closure using the borrow.
        assert_eq!("`color`: green", print());

        // `color` can be borrowed immutably again, because the closure only holds
        // an immutable reference to `color`.
        let _reborrow = &color;
        assert_eq!("`color`: green", print());

        // A move or reborrow is allowed after the final use of `print`
        let _color_moved = color;

        let mut count = 0;
        // A closure to increment `count` could take either `&mut count` or `count`
        // but `&mut count` is less restrictive so it takes that. Immediately
        // borrows `count`.
        //
        // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
        // calling the closure mutates the closure which requires a `mut`.
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        // Call the closure using a mutable borrow.
        inc();

        // The closure still mutably borrows `count` because it is called later.
        // An attempt to reborrow will lead to an error.
        #[cfg(error)] // @formatter:off
        let _reborrow = &count;// @formatter:on

        inc();

        // The closure no longer needs to borrow `&mut count`. Therefore, it is
        // possible to reborrow without an error
        let _count_reborrow = &mut count;

        assert_eq!(2, count);

        // A non-copy type.
        let movable = Box::new(3);

        // `mem::drop` requires `T` so this must take by value. A copy type
        // would copy into the closure leaving the original untouched.
        // A non-copy must move and so `movable` immediately moves into
        // the closure.
        let consume = || {
            println!("`movable`: {:?}", movable);
            drop(movable);
        };

        consume();

        // `consume` consumes the variable so this can only be called once.
        #[cfg(error)]
        consume();

        // `Vec` has non-copy semantics.
        let haystack = vec![1, 2, 3];

        let contains = move |needle| haystack.contains(needle);

        println!("{}", contains(&1));
        println!("{}", contains(&4));

        // compile-time error
        // because borrow checker doesn't allow re-using variable after it
        // has been moved.
        #[cfg(error)]
        println!("They're {} elements in vec", haystack.len());
        // Removing `move` from closure's signature will cause closure
        // to borrow _haystack_ variable immutably, hence _haystack_ is still
        // available and uncommenting above line will not cause an error.
    }
}