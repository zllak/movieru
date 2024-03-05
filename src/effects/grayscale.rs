use crate::{frame::Rgb, Frame};

/// Transform an image to a grayscale version of it
pub struct Grayscale<I> {
    iter: I,
}

impl<I> Grayscale<I> {
    pub(in crate::effects) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for Grayscale<I>
where
    I: Iterator<Item = Frame<Rgb<u8>>>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|frame| {
            frame.transform(|pixel| {
                pixel.0[0] = (pixel.0[0] as f32 * 0.3) as u8;
                pixel.0[1] = (pixel.0[1] as f32 * 0.59) as u8;
                pixel.0[2] = (pixel.0[2] as f32 * 0.11) as u8;
            })
        })
    }
}
