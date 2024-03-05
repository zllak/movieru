use std::slice::{ChunksExact, ChunksExactMut};

pub trait Pixel {
    type ChannelType;
    const CHANNELS: u8;

    fn from_slice(slice: &[Self::ChannelType]) -> &Self;
    fn from_slice_mut(slice: &mut [Self::ChannelType]) -> &mut Self;
}

// ----------------------------------------------------------------------------

#[repr(C)]
pub struct Rgb<T>(pub [T; 3]);

impl<T> Pixel for Rgb<T> {
    type ChannelType = T;

    const CHANNELS: u8 = 3;

    fn from_slice(slice: &[T]) -> &Self {
        assert_eq!(slice.len(), Self::CHANNELS as usize);
        unsafe { &*(slice.as_ptr() as *const Self) }
    }

    fn from_slice_mut(slice: &mut [T]) -> &mut Self {
        assert_eq!(slice.len(), Self::CHANNELS as usize);
        unsafe { &mut *(slice.as_ptr() as *mut Self) }
    }
}

// ----------------------------------------------------------------------------

pub struct PixelsMut<'a, P>
where
    P: Pixel + 'a,
{
    chunks_mut: ChunksExactMut<'a, P::ChannelType>,
}

impl<'a, P> Iterator for PixelsMut<'a, P>
where
    P: Pixel + 'a,
{
    type Item = &'a mut P;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks_mut
            .next()
            .map(|chunk| <P as Pixel>::from_slice_mut(chunk))
    }
}

// ----------------------------------------------------------------------------

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
        // TODO: make sure buffer is big enough for the given size
        Self {
            data,
            width,
            height,
        }
    }

    pub fn pixels(&self) -> ChunksExact<'_, <P as Pixel>::ChannelType> {
        let channels = P::CHANNELS as usize;
        let raw_pixels = &self.data[..(self.width * self.height) as usize * channels];
        raw_pixels.chunks_exact(channels)
    }

    pub fn pixels_mut(&mut self) -> PixelsMut<'_, P> {
        let channels = P::CHANNELS as usize;
        let raw_pixels = &mut self.data[..(self.width * self.height) as usize * channels];
        PixelsMut {
            chunks_mut: raw_pixels.chunks_exact_mut(channels),
        }
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
}

impl IterFrame {
    pub(crate) fn new(
        reader: crate::ffmpeg::FFMpegVideoReader,
        (width, height): (u32, u32),
    ) -> Self {
        Self {
            reader,
            width,
            height,
        }
    }
}

impl Iterator for IterFrame {
    type Item = Frame<Rgb<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader
            .read_frame()
            .ok()?
            .map(|raw_frame| Frame::from_vec(raw_frame, (self.width, self.height)))
    }
}
