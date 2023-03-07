#[derive(Eq, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn new_red() -> Color {
        Color::new(255, 0, 0)
    }

    pub fn new_green() -> Color {
        Color::new(0, 255, 0)
    }

    pub fn new_blue() -> Color {
        Color::new(0, 0, 255)
    }

    /**
     * Returns a new `Color` whose components are the sum of `c1` and `c2`'s components, modulo 256.
     *
     * First, try writing this function the "obvious" way with arithmetic operations. The test for
     * this method (which you can run with `cargo test part1_color` will fail) with a panic.
     *
     * Note which line of the test is causing the panic: why not the other?
     *
     * Then, look through the documentation for `u8` and see if there is a method that will help you.
     * https://doc.rust-lang.org/std/primitive.u8.html
     */
    pub fn cross(c1: &Color, c2: &Color) -> Color {
        Color::new(Self::safe_add(&c1.r, &c2.r), Self::safe_add(&c1.g, &c2.g), Self::safe_add(&c1.b, &c2.b))
        // Color::new((c1.r + c2.r) modulo 256, (c1.g + c2.g) modulo 256, (c1.g + c2.g) modulo 256)
    }

    pub fn safe_add(x: &u8, y: &u8) -> u8 {
        let mut res = 0;
        match x.checked_add(*y) {
            Some(x) => res = x,
            None => res = y - (255 - x) - 1
        }
        res
    }
}
