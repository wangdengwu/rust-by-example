// `allow` required to silence warnings because only
// one variant is used.
#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_enum() {
        let color = Color::RGB(122, 17, 40);
        // TODO ^ Try different variants for `color`

        println!("What color is it?");
        // An `enum` can be destructured using a `match`.
        let index =
            match color {
                Color::Red => {
                    println!("The color is Red!");
                    1
                }
                Color::Blue => {
                    println!("The color is Blue!");
                    2
                }
                Color::Green => {
                    println!("The color is Green!");
                    3
                }
                Color::RGB(r, g, b) => {
                    println!("Red: {}, green: {}, and blue: {}!", r, g, b);
                    4
                }
                Color::HSV(h, s, v) => {
                    println!("Hue: {}, saturation: {}, value: {}!", h, s, v);
                    5
                }
                Color::HSL(h, s, l) => {
                    println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l);
                    6
                }
                Color::CMY(c, m, y) => {
                    println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y);
                    7
                }
                Color::CMYK(c, m, y, k) => {
                    println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                             c, m, y, k);
                    8
                }
                // Don't need another arm because all variants have been examined
            };
        assert_eq!(index, 4);
    }
}