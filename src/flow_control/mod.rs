mod if_else;
mod r#loop;
mod r#while;
mod r#for;
mod r#match;
mod if_let;
mod let_else;
mod while_let;

pub fn test_loop(fizzbuzz: &mut i32, fizz: &mut i32, buzz: &mut i32, counter: &mut i32, n: i32) {
    if n % 15 == 0 {
        *fizzbuzz += 1;
    } else if n % 3 == 0 {
        *fizz += 1;
    } else if n % 5 == 0 {
        *buzz += 1;
    } else {
        *counter += 1;
    }
}