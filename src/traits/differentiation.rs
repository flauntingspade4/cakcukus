use num_traits::{
    identities::{one, zero},
    Num, Pow,
};

#[cfg(feature = "built_in_types")]
use crate::{traits::TermTrait, Polynomial, Term};

/// Describes a type's ability to do basic differentiation
pub trait Differentiation<T: Num + Pow<T, Output = T> + Copy> {
    /// Differentiates self, with respect to a given x
    fn differentiate(&self, x: &T) -> T;
    /// Differentiates a copy of self, and returns the copy
    fn differentiate_self(&self) -> Self;
}

#[cfg(feature = "built_in_types")]
impl<T> Differentiation<T> for Polynomial<T>
where
    T: Num + Pow<T, Output = T> + Copy + PartialOrd,
{
    fn differentiate(&self, x: &T) -> T {
        let mut total = zero();
        for term in self.0.iter() {
            total = total + term.differentiate(&x);
        }
        total
    }
    fn differentiate_self(&self) -> Self {
        let mut all = Vec::with_capacity(self.0.len());
        for term in self.0.iter() {
            all.push(term.differentiate_self())
        }
        Self(all)
    }
}

#[cfg(feature = "built_in_types")]
impl<T> Differentiation<T> for Term<T>
where
    T: Num + Pow<T, Output = T> + Copy,
{
    fn differentiate(&self, x: &T) -> T {
        self.differentiate_self().sum_with_respect_to(x)
    }
    fn differentiate_self(&self) -> Self {
        Self::new(self.coefficient * self.exponent, self.exponent - one())
    }
}
