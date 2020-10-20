use num_traits::{identities::zero, Num, Pow};

use crate::{Polynomial, Term};

/// A trait describing a type's ability to be
/// treated as a term in a polynomial
pub trait TermTrait<T>
where
    T: Num + Pow<T, Output = T> + Copy,
{
    /// Sums between two given upper and lower bounds
    fn sum_between(&self, lower: T, upper: T) -> T {
        self.sum_with_respect_to(&upper) - self.sum_with_respect_to(&lower)
    }
    /// Sums the term, with respect to a given x
    /// ```
    /// use cakcukus::{terms, traits::TermTrait, Term};
    ///
    /// let term = terms!(5., 2.); // 5x^2
    ///
    /// let summed = term.sum_with_respect_to(&2.);
    ///
    /// // 5(2)^2 = 5(4) = 20
    ///
    /// assert_eq!(20., summed);
    /// ```
    fn sum_with_respect_to(&self, x: &T) -> T;
}

impl<T> TermTrait<T> for Polynomial<T>
where
    T: Num + Pow<T, Output = T> + Copy,
{
    fn sum_with_respect_to(&self, x: &T) -> T {
        let mut total = zero();
        for term in self.0.iter() {
            total = total + term.sum_with_respect_to(x);
        }
        total
    }
}

impl<T> TermTrait<T> for Term<T>
where
    T: Num + Pow<T, Output = T> + Copy,
{
    fn sum_with_respect_to(&self, x: &T) -> T {
        let sum = self.coefficient * x.pow(self.exponent);
        if !sum.eq(&sum) {
            zero()
        } else {
            sum
        }
    }
}
