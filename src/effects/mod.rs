use crate::Frame;
use image::{Pixel, Rgb};

mod grayscale;
use self::grayscale::Grayscale;

mod crop;
use self::crop::Crop;

mod resize;
use self::resize::Resize;

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
    fn crop<P>(self, x: u32, y: u32, width: u32, height: u32) -> Crop<Self>
    where
        Self: Sized,
        Self: Iterator<Item = Frame<P>>,
        P: Pixel,
    {
        Crop::new(self, x, y, width, height)
    }

    /// Resize the frame.
    /// TODO: shall we support speaking in term of aspect ratio?
    fn resize<P>(self, width: u32, height: u32) -> Resize<Self>
    where
        Self: Sized,
        Self: Iterator<Item = Frame<P>>,
        P: Pixel,
    {
        Resize::new(self, width, height)
    }
}

/// Blank implementation of trait EffectsExt for iterators on `Frame`.
impl<I: ?Sized, P: Pixel> EffectsExt for I where I: Iterator<Item = Frame<P>> {}
