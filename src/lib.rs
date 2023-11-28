use bmp::{Image, Pixel};

// TODO: how do I not make this public?
// - maybe one of this: pub(in path) , pub(crate) , pub(super) , and pub(self)
// link: https://doc.rust-lang.org/reference/visibility-and-privacy.html
pub const BUFFER_BETWEEN_CHARS: usize = 1;
pub const BUFFER_FOR_FIRST_AND_LAST_CHARS: usize = 1;
pub const PIXEL_CHAR_WIDTH: usize = 3;
pub const PIXEL_CHAR_HEIGHT: usize = 4;

pub struct Pencil {
    pub x: u32,
    pub y: u32,
}

impl Pencil {
    pub fn move_x(mut self, x: u32) -> Self {
        self.x = self.x + x;
        self
    }
    pub fn move_y(mut self, y: u32) -> Self {
        self.y = self.y + y;
        self
    }
}

#[macro_export]
macro_rules! write {
    ($word:expr, $path:expr) => {{
        let slen: usize = $word.len();
        let width = slen
            * (wripix::PIXEL_CHAR_WIDTH + wripix::BUFFER_BETWEEN_CHARS) + wripix::BUFFER_FOR_FIRST_AND_LAST_CHARS;
        let height = wripix::PIXEL_CHAR_HEIGHT + 2;
		// NOTE: bmp::Image::new() needs u32, may present potential future problems as I'm using usize
		// NOTE: may need to roll own library or fork bmp and maintain own library (as bmp isn't exactly maintained: latest update is 4 years ago)
        let mut image: Image = Image::new(
            u32::try_from(width).unwrap(),
            u32::try_from(height).unwrap(),
        );
        let colour: Pixel = Pixel::new(255, 0, 0);
		let mut pencil: wripix::Pencil = wripix::Pencil {
			x: 1, y: 1,
		};
        for c in $word.chars() {
            match c {
                'A' => {
                    wripix::uppercase_a(&mut image, colour, &pencil);
                }
                'B' => {
                    wripix::uppercase_b(&mut image, colour, &pencil);
                }
                'C' => {
                    wripix::uppercase_c(&mut image, colour, &pencil);
                }
                'D' => {
                    wripix::uppercase_d(&mut image, colour, &pencil);
                }
                'E' => {
                    wripix::uppercase_e(&mut image, colour, &pencil);
                }
                'F' => {
                    wripix::uppercase_f(&mut image, colour, &pencil);
                }
                'G' => {
                    wripix::uppercase_g(&mut image, colour, &pencil);
                }
                'H' => {
                    wripix::uppercase_h(&mut image, colour, &pencil);
                }
                'I' => {
                    wripix::uppercase_i(&mut image, colour, &pencil);
                }
                'J' => {
                    wripix::uppercase_j(&mut image, colour, &pencil);
                }
                'K' => {
                    wripix::uppercase_k(&mut image, colour, &pencil);
                }
                'L' => {
                    wripix::uppercase_l(&mut image, colour, &pencil);
                }
                'M' => {
                    wripix::uppercase_m(&mut image, colour, &pencil);
                }
                'N' => {
                    wripix::uppercase_n(&mut image, colour, &pencil);
                }
                'O' => {
                    wripix::uppercase_o(&mut image, colour, &pencil);
                }
                'P' => {
                    wripix::uppercase_p(&mut image, colour, &pencil);
                }
                'Q' => {
                    wripix::uppercase_q(&mut image, colour, &pencil);
                }
                'R' => {
                    wripix::uppercase_r(&mut image, colour, &pencil);
                }
                'S' => {
                    wripix::uppercase_s(&mut image, colour, &pencil);
                }
                'T' => {
                    wripix::uppercase_t(&mut image, colour, &pencil);
                }
                'U' => {
                    wripix::uppercase_u(&mut image, colour, &pencil);
                }
                'V' => {
                    wripix::uppercase_v(&mut image, colour, &pencil);
                }
                'W' => {
                    wripix::uppercase_w(&mut image, colour, &pencil);
                }
                'X' => {
                    wripix::uppercase_x(&mut image, colour, &pencil);
                }
                'Y' => {
                    wripix::uppercase_y(&mut image, colour, &pencil);
                }
                'Z' => {
                    wripix::uppercase_z(&mut image, colour, &pencil);
                }
                'a' => {
                    wripix::lowercase_a(&mut image, colour, &pencil);
                }
                'b' => {
                    wripix::lowercase_b(&mut image, colour, &pencil);
                }
                'c' => {
                    wripix::lowercase_c(&mut image, colour, &pencil);
                }
                'd' => {
                    wripix::lowercase_d(&mut image, colour, &pencil);
                }
                'e' => {
                    wripix::lowercase_e(&mut image, colour, &pencil);
                }
                'f' => {
                    wripix::lowercase_f(&mut image, colour, &pencil);
                }
                'g' => {
                    wripix::lowercase_g(&mut image, colour, &pencil);
                }
                'h' => {
                    wripix::lowercase_h(&mut image, colour, &pencil);
                }
                'i' => {
                    wripix::lowercase_i(&mut image, colour, &pencil);
                }
                'j' => {
                    wripix::lowercase_j(&mut image, colour, &pencil);
                }
                'k' => {
                    wripix::lowercase_k(&mut image, colour, &pencil);
                }
                'l' => {
                    wripix::lowercase_l(&mut image, colour, &pencil);
                }
                'm' => {
                    wripix::lowercase_m(&mut image, colour, &pencil);
                }
                'n' => {
                    wripix::lowercase_n(&mut image, colour, &pencil);
                }
                'o' => {
                    wripix::lowercase_o(&mut image, colour, &pencil);
                }
                'p' => {
                    wripix::lowercase_p(&mut image, colour, &pencil);
                }
                'q' => {
                    wripix::lowercase_q(&mut image, colour, &pencil);
                }
                'r' => {
                    wripix::lowercase_r(&mut image, colour, &pencil);
                }
                's' => {
                    wripix::lowercase_s(&mut image, colour, &pencil);
                }
                't' => {
                    wripix::lowercase_t(&mut image, colour, &pencil);
                }
                'u' => {
                    wripix::lowercase_u(&mut image, colour, &pencil);
                }
                'v' => {
                    wripix::lowercase_v(&mut image, colour, &pencil);
                }
                'w' => {
                    wripix::lowercase_w(&mut image, colour, &pencil);
                }
                'x' => {
                    wripix::lowercase_x(&mut image, colour, &pencil);
                }
                'y' => {
                    wripix::lowercase_y(&mut image, colour, &pencil);
                }
                'z' => {
                    wripix::lowercase_z(&mut image, colour, &pencil);
                }
                _ => {}
            }
			pencil = pencil.move_x(u32::try_from(wripix::PIXEL_CHAR_WIDTH + wripix::BUFFER_BETWEEN_CHARS).unwrap());
        }
		image.save($path).expect("This should save correctly.");
    }};
}

pub fn lowercase_a(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn lowercase_b(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn lowercase_c(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
}

pub fn lowercase_d(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn lowercase_e(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn lowercase_f(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
}

pub fn lowercase_g(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 4, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 4, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn lowercase_h(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn lowercase_i(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
}

pub fn lowercase_j(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 4, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
}

pub fn lowercase_k(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn lowercase_l(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
}

pub fn lowercase_m(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn lowercase_n(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn lowercase_o(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn lowercase_p(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 4, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn lowercase_q(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 4, colour);
}

pub fn lowercase_r(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
}

pub fn lowercase_s(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
}

pub fn lowercase_t(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn lowercase_u(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn lowercase_v(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn lowercase_w(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn lowercase_x(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn lowercase_y(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 4, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn lowercase_z(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 4, colour);
}

pub fn uppercase_a(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_b(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_c(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_d(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn uppercase_e(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_f(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
}

pub fn uppercase_g(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_h(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_i(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_j(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn uppercase_k(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_l(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_m(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_n(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_o(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn uppercase_p(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
}

pub fn uppercase_q(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_r(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_s(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_t(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
}

pub fn uppercase_u(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_v(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn uppercase_w(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 2, colour);
}

pub fn uppercase_x(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

pub fn uppercase_y(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
}

pub fn uppercase_z(image: &mut Image, colour: Pixel, pencil: &Pencil) {
    image.set_pixel(pencil.x + 0, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 2, colour);
    image.set_pixel(pencil.x + 0, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 1, pencil.y + 3, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 0, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 1, colour);
    image.set_pixel(pencil.x + 2, pencil.y + 3, colour);
}

// TODO: border
// pub fn draw_user_sq(image: &mut Image, width: u32, height: u32) {
//     for w in 0..width {
//         image.set_pixel(w, 0, Pixel::new(255, 0, 0));
//     }
//     for w in 0..width {
//         image.set_pixel(w, height - 1, Pixel::new(255, 0, 0));
//     }
//     for h in 0..height {
//         image.set_pixel(0, h, Pixel::new(255, 0, 0));
//     }
//     for h in 0..height {
//         image.set_pixel(width - 1, h, Pixel::new(255, 0, 0));
//     }
// }
