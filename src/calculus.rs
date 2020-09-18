use crate::term::Term;

use num_traits::{Num, Pow};

/// A trait describing a type's ability to perform basic calculus
pub trait Calculus<T>
where
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    /// Intergrates a copy of self, and returns the copy
    fn intergrated_self(&self) -> Self;
    /// differentiates a copy of self, and returns the copy
    fn differentiate_self(&self) -> Self;
    /// Returns self, intergrated to the upper and lower bound
    fn intergrate(&self, lower: T, upper: T) -> T;
    /// Sums between two given upper and lower bounds
    fn sum_between(&self, lower: T, upper: T) -> T {
        self.sum_with_respect_to(&upper) - self.sum_with_respect_to(&lower)
    }
    /// Sums the term, with respect to a given x
    fn sum_with_respect_to(&self, x: &T) -> T;
}

impl<T> Calculus<T> for Vec<Term<T>>
where
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    fn intergrated_self(&self) -> Self {
        let mut all = Vec::with_capacity(self.len());
        for x in self.iter() {
            all.push(x.intergrated_self())
        }
        all
    }
    fn differentiate_self(&self) -> Self {
        let mut all = Vec::with_capacity(self.len());
        for x in self.iter() {
            all.push(x.differentiate_self())
        }
        all
    }
    fn intergrate(&self, lower: T, upper: T) -> T {
        let mut total = T::from(0);
        for x in self.iter() {
            total = total + x.sum_with_respect_to(&upper) - x.sum_with_respect_to(&lower);
        }
        total
    }
    fn sum_with_respect_to(&self, x: &T) -> T {
        return self
            .iter()
            .fold(T::from(0), |i, j| i + j.sum_with_respect_to(x));
    }
}

impl<T> Calculus<T> for Term<T>
where
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    fn intergrated_self(&self) -> Self {
        let exponent = self.exponent + T::from(1);
        Self::new(
            self.coefficient / exponent,
            exponent,
        )
    }
    fn differentiate_self(&self) -> Self {
        Self::new(
            self.coefficient * self.exponent,
            self.exponent - T::from(1)
        )
    }
    fn intergrate(&self, upper: T, lower: T) -> T {
        let intergrated = self.intergrated_self();
        intergrated.sum_with_respect_to(&upper) - intergrated.sum_with_respect_to(&lower)
    }

    fn sum_with_respect_to(&self, x: &T) -> T {
        return self.coefficient * x.pow(self.exponent);
    }
}
