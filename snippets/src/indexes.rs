use std::vec::Vec;
use std::ops::{Index, IndexMut};

struct Image<P> {
    width: usize,
    pixels: Vec<P>
}

impl<P> Image<P>
    where P: Default + Copy {
        fn new(width: usize, height: usize) -> Image<P> {
            Image {
                width,
                pixels: vec![P::default(); width * height]
            }
        }
    }

impl<P> Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start .. start + self.width]
    }
}

impl<P> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start .. start + self.width]
    }
}