struct Cardinal;

struct BlueJay;

struct Turkey;

trait Red {}

trait Blue {}

impl Red for Cardinal {}

impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T) -> &'static str { "red" }

fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testcase_empty() {
        let cardinal = Cardinal;
        let blue_jay = BlueJay;
        let _turkey = Turkey;

        // `red()` won't work on a blue jay nor vice versa
        // because of the bounds.
        assert_eq!("red", red(&cardinal));
        assert_eq!("blue", blue(&blue_jay));
        //println!("A turkey is {}", red(&_turkey));
        // ^ TODO: Try uncommenting this line.
    }
}