use num_traits::{Num, Pow};

use crate::Term;

pub trait TermTrait<T: Num + Pow<T, Output = T> + From<u8> + Copy> {
    /// Sums between two given upper and lower bounds
    fn sum_between(&self, lower: T, upper: T) -> T {
        self.sum_with_respect_to(&upper) - self.sum_with_respect_to(&lower)
    }
    /// Sums the term, with respect to a given x
    fn sum_with_respect_to(&self, x: &T) -> T;
}

impl<J, T> TermTrait<T> for Vec<J>
where
    J: TermTrait<T>,
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    fn sum_with_respect_to(&self, x: &T) -> T {
        let mut total = T::from(0);
        for term in self.iter() {
            total = total + term.sum_with_respect_to(x);
        }
        total
    }
}

impl<T> TermTrait<T> for Term<T>
where
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    fn sum_with_respect_to(&self, x: &T) -> T {
        self.coefficient * x.pow(self.exponent)
    }
}
