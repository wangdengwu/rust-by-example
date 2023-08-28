use std::fmt::Display;

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_city_display() {
        let mut v = vec![];
        for city in [
            City {
                name: "Dublin",
                lat: 53.347778,
                lon: -6.259722,
            },
            City {
                name: "Oslo",
                lat: 59.95,
                lon: 10.75,
            },
        ] {
            v.push(format!("{}", city));
        }
        assert_eq!(
            v,
            [
                "Dublin: 53.348°N 6.260°W",
                "Oslo: 59.950°N 10.750°E",
            ]
        )
    }

    #[test]
    fn test_color_display() {
        let mut v = vec![];
        for color in [
            Color { red: 128, green: 255, blue: 90 },
            Color { red: 0, green: 3, blue: 254 },
            Color { red: 0, green: 0, blue: 0 },
        ] {
            v.push(format!("{}", color));
        }
        assert_eq!(
            v,
            [
                "#80FF5A",
                "#0003FE",
                "#000000",
            ]
        )
    }
}