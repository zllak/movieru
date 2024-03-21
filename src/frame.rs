use image::{ImageBuffer, Pixel, Rgb};

pub struct Frame<P>
where
    P: Pixel,
{
    data: ImageBuffer<P, Vec<P::Subpixel>>,
    width: u32,
    height: u32,
}

impl<P> Frame<P>
where
    P: Pixel,
{
    pub fn from_vec(data: Vec<<P as Pixel>::Subpixel>, (width, height): (u32, u32)) -> Self {
        let data =
            ImageBuffer::from_raw(width, height, data).expect("cannot instanciate image buffer");

        Self {
            data,
            width,
            height,
        }
    }

    /// Returns the raw buffer
    /// TODO: this is not ideal, we can do better
    pub fn as_raw(&self) -> &Vec<<P as Pixel>::Subpixel> {
        self.data.as_raw()
    }

    /// Returns the underlying image::ImageBuffer, so we can use the image::imageops
    /// methods.
    pub(crate) fn image(&self) -> &ImageBuffer<P, Vec<P::Subpixel>> {
        &self.data
    }

    /// Transforms the current frame using the given function.
    /// Transformation is applied on pixels, so the format of the frame cannot
    /// change.
    pub fn transform<F, O>(self, func: F) -> Frame<O>
    where
        F: Fn(&P) -> O,
        O: Pixel<Subpixel = P::Subpixel>,
    {
        let mut buffer = Vec::with_capacity(self.data.len());

        for pixel in self.data.pixels() {
            buffer.extend(func(pixel).channels());
        }

        Frame::from_vec(buffer, (self.width, self.height))
    }

    /// Map the given frame with the given function.
    /// This allows for changing the format of the frame.
    pub fn map<F, O>(self, func: F) -> Frame<O>
    where
        F: Fn(Self) -> Frame<O>,
        O: Pixel<Subpixel = P::Subpixel>,
    {
        func(self)
    }
}

// ----------------------------------------------------------------------------

pub struct IterFrame {
    reader: crate::ffmpeg::FFMpegVideoReader,
    width: u32,
    height: u32,
    nb_frames: usize,
}

impl IterFrame {
    pub(crate) fn new(
        reader: crate::ffmpeg::FFMpegVideoReader,
        (width, height): (u32, u32),
        nb_frames: usize,
    ) -> Self {
        Self {
            reader,
            width,
            height,
            nb_frames,
        }
    }
}

impl Iterator for IterFrame {
    type Item = Frame<Rgb<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        // FIXME: here we silently ignore errors, we might want to change that
        self.reader
            .read_frame()
            .ok()?
            .map(|raw_frame| Frame::from_vec(raw_frame, (self.width, self.height)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.nb_frames, Some(self.nb_frames))
    }
}
