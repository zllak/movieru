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

    /// Transforms the current frame using the given function.
    /// Only assumes rgb24 for now, and does not try to be smart when transforming.
    pub fn transform<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut P),
    {
        for pixel in self.data.pixels_mut() {
            func(pixel);
        }

        self
    }

    // DO NOT USE, just to test something
    fn grayscale(self) -> Frame<image::Luma<<P as Pixel>::Subpixel>>
    where
        P: Pixel,
    {
        let gray = image::imageops::grayscale(&self.data);
        Frame::from_vec(gray.into_raw(), (self.width, self.height))
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
