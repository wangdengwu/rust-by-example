use std::fmt::{Display, Formatter, Result};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "( {} {} )\n ( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse((1, true)), (true, 1));
    }

    #[test]
    fn test_tuples_by_index() {
        let long_tuple = (1u8, 2u16, 3u32, 4u64,
                          -1i8, -2i16, -3i32, -4i64,
                          0.1f32, 0.2f64,
                          'a', true);
        assert_eq!(long_tuple.0, 1);
        assert_eq!(long_tuple.11, true);
    }

    #[test]
    fn test_tuple_of_tuples() {
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
        let o = format!("tuple of tuples: {:?}", tuple_of_tuples);
        assert_eq!(o, "tuple of tuples: ((1, 2, 2), (4, -1), -2)");

        // But long Tuples (more than 12 elements) cannot be printed.
        let _too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
        // println!("Too long tuple: {:?}", too_long_tuple);
        // assert_eq!(too_long_tuple, (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13));
    }

    #[test]
    #[allow(unused_parens)]
    fn test_create_one_element() {
        let one = (1, );
        assert_eq!(1, one.0);
        let one = (5u32);
        assert_eq!(5, one);
    }

    #[test]
    fn test_destructured() {
        let tuple = (1, "hello", 4.5, true);
        let (a, b, c, d) = tuple;
        let o = format!("{a}, {b}, {c:?}, {d}");
        assert_eq!(o, "1, hello, 4.5, true");
    }

    #[test]
    fn test_matrix_display() {
        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        let o = format!("{}", matrix);
        assert_eq!(o, "( 1.1 1.2 )\n ( 2.1 2.2 )");
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        let o = format!("{}", transpose(matrix));
        assert_eq!(o, "( 1.1 2.1 )\n ( 1.2 2.2 )");
    }
}