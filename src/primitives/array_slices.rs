use std::mem::size_of_val;

fn analyze_slice(slice: &[i32]) -> (i32, usize) {
    println!("Array occupies {} bytes", size_of_val(&slice));
    (slice[0], slice.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_index_len() {
        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(xs[0], 1);
        assert_eq!(xs[1], 2);
        assert_eq!(xs.len(), 5);
    }

    #[test]
    fn test_analyze_slice() {
        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        let ys: [i32; 500] = [0; 500];
        assert_eq!((1, 5), analyze_slice(&xs));
        assert_eq!((0, 3), analyze_slice(&ys[1..4]));
    }

    #[test]
    fn test_empty_slice() {
        let empty_array: [u32; 0] = [];
        assert_eq!(&empty_array, &[]);
        assert_eq!(&empty_array, &[][..]); // Same but more verbose
    }

    #[test]
    fn test_for_in_array() {
        let xs: [usize; 5] = [1, 2, 3, 4, 5];
        for i in 1..xs.len() + 1 {
            match xs.get(i - 1) {
                Some(v) => assert_eq!(&i, v),
                None => assert_eq!(xs.len() + 1, i),
            }
        }
        // Out of bound indexing on array causes compile time error.
        // println!("{}", xs[5]);
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_bound() {
        // Out of bound indexing on slice causes runtime error.
        println!("{}", [1, 2, 3, 4, 5][..][5]);
    }
}