use std::ops::{Index, IndexMut};
use std::vec::Vec;

#[derive(PartialEq)]
struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P> Image<P>
where
    P: Default + Copy,
{
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

#[test]
fn image_index_test() {
    let mut img = Image::new(5, 5);
    img[0][0] = 3;
    img[0][1] = 4;
    img[1][0] = 15;
    img[2][1] = 20;

    let pixels = &img.pixels;
    assert_eq!(pixels[0], 3);
    assert_eq!(pixels[1], 4);
    assert_eq!(pixels[5], 15);
    assert_eq!(pixels[11], 20);
}
