use std::slice::{ChunksExact, ChunksExactMut};

pub trait Pixel {
    type ChannelType;
    const CHANNELS: u8;
    const NAME: &'static str;

    fn from_slice(slice: &[Self::ChannelType]) -> &Self;
    fn from_slice_mut(slice: &mut [Self::ChannelType]) -> &mut Self;
}

// ----------------------------------------------------------------------------

#[repr(C)]
pub struct Rgb<T>(pub [T; 3]);

impl<T> Pixel for Rgb<T> {
    type ChannelType = T;

    const CHANNELS: u8 = 3;
    const NAME: &'static str = "rgb24";

    fn from_slice(slice: &[T]) -> &Self {
        assert_eq!(slice.len(), Self::CHANNELS as usize);
        unsafe { &*(slice.as_ptr() as *const Self) }
    }

    fn from_slice_mut(slice: &mut [T]) -> &mut Self {
        assert_eq!(slice.len(), Self::CHANNELS as usize);
        unsafe { &mut *(slice.as_ptr() as *mut Self) }
    }
}

#[repr(C)]
pub struct Yuv420p<T>(pub [T; 3]);

impl<T> Pixel for Yuv420p<T> {
    type ChannelType = T;

    const CHANNELS: u8 = 3;
    const NAME: &'static str = "yuv420p";

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

pub struct Pixels<'a, P>
where
    P: Pixel + 'a,
{
    pub chunks: ChunksExact<'a, P::ChannelType>,
    pub size_hint: usize,
}

impl<'a, P> Iterator for Pixels<'a, P>
where
    P: Pixel + 'a,
{
    type Item = &'a P;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks
            .next()
            .map(|chunk| <P as Pixel>::from_slice(chunk))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size_hint, Some(self.size_hint))
    }
}

// ----------------------------------------------------------------------------

pub struct PixelsMut<'a, P>
where
    P: Pixel + 'a,
{
    pub chunks_mut: ChunksExactMut<'a, P::ChannelType>,
    pub size_hint: usize,
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size_hint, Some(self.size_hint))
    }
}
