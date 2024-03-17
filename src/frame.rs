use crate::{Pixel, Pixels, PixelsMut, Rgb};

pub struct Frame<P>
where
    P: Pixel,
{
    data: Vec<<P as Pixel>::ChannelType>,
    width: u32,
    height: u32,
}

impl<P> Frame<P>
where
    P: Pixel,
{
    pub fn from_vec(data: Vec<<P as Pixel>::ChannelType>, (width, height): (u32, u32)) -> Self {
        assert_eq!(
            data.capacity(),
            (width * height * <P as Pixel>::CHANNELS as u32) as usize,
            "buffer is too small"
        );

        Self {
            data,
            width,
            height,
        }
    }

    pub fn pixels(&self) -> Pixels<'_, P> {
        let channels = P::CHANNELS as usize;
        let size_hint = (self.width * self.height) as usize;
        let raw_pixels = &self.data[..size_hint * channels];
        Pixels {
            chunks: raw_pixels.chunks_exact(channels),
            size_hint,
        }
    }

    pub fn pixels_mut(&mut self) -> PixelsMut<'_, P> {
        let channels = P::CHANNELS as usize;
        let size_hint = (self.width * self.height) as usize;
        let raw_pixels = &mut self.data[..size_hint * channels];
        PixelsMut {
            chunks_mut: raw_pixels.chunks_exact_mut(channels),
            size_hint,
        }
    }

    /// Returns the raw buffer
    /// TODO: this is not ideal, we can do better
    pub fn raw(&self) -> Vec<<P as Pixel>::ChannelType>
    where
        <P as Pixel>::ChannelType: Clone,
    {
        self.data[..].to_vec()
    }

    /// Transforms the current frame using the given function.
    /// Only assumes rgb24 for now, and does not try to be smart when transforming.
    pub fn transform<F>(mut self, func: F) -> Self
    where
        F: Fn(&mut P),
    {
        for pixel in self.pixels_mut() {
            func(pixel);
        }

        self
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
