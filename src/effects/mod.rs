use crate::Frame;
use image::Rgb;

mod grayscale;
use self::grayscale::Grayscale;

mod crop;
use self::crop::Crop;

// TODO: only handles Rgb<u8> for now

/// Trait extension to add effects for iterators on `Frame`.
pub trait EffectsExt: Iterator {
    /// Applies a grayscale effect to the frame
    fn grayscale(self) -> Grayscale<Self>
    where
        Self: Sized,
        Self: Iterator<Item = Frame<Rgb<u8>>>,
    {
        Grayscale::new(self)
    }

    /// Crop the frame at (x, y) to a new (width, height)
    /// TODO: shall we handle aspect ratio instead of raw width/height?
    fn crop(self, x: u32, y: u32, width: u32, height: u32) -> Crop<Self>
    where
        Self: Sized,
        Self: Iterator<Item = Frame<Rgb<u8>>>,
    {
        Crop::new(self, x, y, width, height)
    }
}

/// Blank implementation of trait EffectsExt for iterators on `Frame`.
impl<I: ?Sized> EffectsExt for I where I: Iterator<Item = Frame<Rgb<u8>>> {}
