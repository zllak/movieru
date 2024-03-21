use crate::Frame;
use image::Pixel;

pub struct Crop<I> {
    iter: I,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl<I> Crop<I> {
    pub(in crate::effects) fn new(iter: I, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            iter,
            x,
            y,
            width,
            height,
        }
    }
}

impl<I, P> Iterator for Crop<I>
where
    P: Pixel + 'static,
    I: Iterator<Item = Frame<P>>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|frame| {
            let new_image =
                image::imageops::crop_imm(frame.image(), self.x, self.y, self.width, self.height)
                    .to_image();
            let dimensions = new_image.dimensions();

            Frame::from_vec(new_image.into_raw(), dimensions)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
