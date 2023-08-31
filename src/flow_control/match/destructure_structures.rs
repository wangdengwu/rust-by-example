struct Foo {
    x: (u32, u32),
    y: u32,
}

#[cfg(test)]
mod tests {
    use super::Foo;

    #[test]
    fn test_struct() {
        // Try changing the values in the struct to see what happens
        let foo = Foo { x: (1, 2), y: 3 };
        let y =
            match foo {
                Foo { x: (1, b), y } => {
                    println!("First of x is 1, b = {},  y = {} ", b, y);
                    y
                }

                // you can destructure structs and rename the variables,
                // the order is not important
                Foo { y: 2, x: i } => {
                    println!("y is 2, i = {:?}", i);
                    2
                }

                // and you can also ignore some variables:
                Foo { y, .. } => {
                    println!("y = {}, we don't care about x", y);
                    y
                }
                // this will give an error: pattern does not mention field `x`
                #[cfg(error)]
                Foo { y } => println!("y = {}", y),
            };
        assert_eq!(y, 3);

        let faa = Foo { x: (1, 2), y: 3 };

        // You do not need a match block to destructure structs:
        let Foo { x: x0, y: y0 } = faa;
        println!("Outside: x0 = {x0:?}, y0 = {y0}");
        assert_eq!(x0, (1, 2));
        assert_eq!(y0, 3);
    }
}