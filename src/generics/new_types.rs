struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_types() {
        let age = Years(5);
        let age_days = age.to_days();
        assert_eq!(false, old_enough(&age));
        assert_eq!(false, old_enough(&age_days.to_years()));
        // println!("Old enough {}", old_enough(&age_days));
    }
}