use num_traits::{Num, Pow};

mod differentiation;
mod integration;
mod term;

pub use differentiation::Differentiation;
pub use integration::Integration;
pub use term::TermTrait;

/// A trait describing a type's ability to perform basic calculus,
/// automatically implemented for any type that implements
/// [Intergration](trait.Intergration.html), [Differentiation](trait.Differentiation.html)
/// and [TermTrait](trait.TermTrait.html).
pub trait Calculus<T>: Differentiation<T> + Integration<T> + TermTrait<T>
where
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
}

impl<I, T> Calculus<T> for I
where
    I: Differentiation<T> + Integration<T> + TermTrait<T>,
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
}
