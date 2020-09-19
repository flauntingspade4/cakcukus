use num_traits::{Num, Pow};

mod differentiation;
mod intergration;
mod term;

pub use differentiation::Differentiation;
pub use intergration::Intergration;
pub use term::TermTrait;

/// A trait describing a type's ability to perform basic calculus,
/// automatically implemented for any type that implements
/// [Intergration](trait.Intergration.html), [Differentiation](trait.Differentiation.html)
/// and [TermTrait](trait.TermTrait.html).
pub trait Calculus<T>: Differentiation<T> + Intergration<T> + TermTrait<T>
where
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
}

impl<I, T> Calculus<T> for I
where
    I: Differentiation<T> + Intergration<T> + TermTrait<T>,
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
}
