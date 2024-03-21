use crate::Frame;
use image::{imageops::FilterType, Pixel};

pub struct Resize<I> {
    iter: I,
    width: u32,
    height: u32,
}

impl<I> Resize<I> {
    pub(in crate::effects) fn new(iter: I, width: u32, height: u32) -> Self {
        Resize {
            iter,
            width,
            height,
        }
    }
}

/// TODO: this has horrible performance, resizing a relatively small frame to
/// a short 9:16 format (1920x1080) using lanczos takes more than 1 second.
impl<I, P> Iterator for Resize<I>
where
    P: Pixel + 'static, // static lifetime is required by `resize`
    I: Iterator<Item = Frame<P>>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|frame| {
            let new_image = image::imageops::resize(
                frame.image(),
                self.width,
                self.height,
                FilterType::Lanczos3,
            );
            let dimensions = new_image.dimensions();

            Frame::from_vec(new_image.into_raw(), dimensions)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
