use crate::{frame::Rgb, Frame};
mod grayscale;
use self::grayscale::Grayscale;

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
}

/// Blank implementation of trait EffectsExt for iterators on `Frame`.
impl<I: ?Sized> EffectsExt for I where I: Iterator<Item = Frame<Rgb<u8>>> {}
