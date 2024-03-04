use crate::Frame;

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
    I: Iterator<Item = Frame>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|frame| {
            frame.transform(|r, g, b| {
                (
                    (r as f32 * 0.3) as u8,
                    (g as f32 * 0.59) as u8,
                    (b as f32 * 0.11) as u8,
                )
            })
        })
    }
}
