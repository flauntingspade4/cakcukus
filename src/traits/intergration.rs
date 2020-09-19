use num_traits::{Num, Pow};

use crate::Term;

use super::TermTrait;

pub trait Intergration<T: Num + Pow<T, Output = T> + From<u8> + Copy> {
    /// Intergrates a copy of self, and returns the copy
    fn intergrated_self(&self) -> Self;
    /// Returns self, intergrated to the upper and lower bound
    fn intergrate(&self, lower: T, upper: T) -> T;
}

impl<J, T> Intergration<T> for Vec<J>
where
    J: Intergration<T> + TermTrait<T>,
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    fn intergrated_self(&self) -> Self {
        let mut all = Vec::with_capacity(self.len());
        for x in self.iter() {
            all.push(x.intergrated_self())
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
}

impl<T> Intergration<T> for Term<T>
where
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    fn intergrated_self(&self) -> Self {
        let exponent = self.exponent + T::from(1);
        Self::new(self.coefficient / exponent, exponent)
    }

    fn intergrate(&self, upper: T, lower: T) -> T {
        let intergrated = self.intergrated_self();
        intergrated.sum_with_respect_to(&upper) - intergrated.sum_with_respect_to(&lower)
    }
}
