use num_traits::{Num, Pow};

use crate::Term;

use super::TermTrait;

/// Describes a type's ability to do basic differentiation
pub trait Differentiation<T: Num + Pow<T, Output = T> + From<u8> + Copy> {
    /// differentiates a copy of self, and returns the copy
    fn differentiate_self(&self) -> Self;
}

impl<J, T> Differentiation<T> for Vec<J>
where
    J: Differentiation<T> + TermTrait<T>,
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    fn differentiate_self(&self) -> Self {
        let mut all = Vec::with_capacity(self.len());
        for x in self.iter() {
            all.push(x.differentiate_self())
        }
        all
    }
}

impl<T> Differentiation<T> for Term<T>
where
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    fn differentiate_self(&self) -> Self {
        Self::new(self.coefficient * self.exponent, self.exponent - T::from(1))
    }
}
