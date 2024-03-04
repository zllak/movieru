use crate::Frame;

mod grayscale;
use self::grayscale::Grayscale;

/// Trait extension to add effects for iterators on `Frame`.
pub trait EffectsExt: Iterator {
    /// Applies a grayscale effect to the frame
    fn grayscale(self) -> Grayscale<Self>
    where
        Self: Sized,
        Self: Iterator<Item = Frame>,
    {
        Grayscale::new(self)
    }
}

/// Blank implementation of trait EffectsExt for iterators on `Frame`.
impl<I: ?Sized> EffectsExt for I where I: Iterator<Item = Frame> {}
