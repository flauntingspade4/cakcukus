use num_traits::{
    identities::{one, zero},
    Num, Pow,
};

use crate::{traits::TermTrait, Polynomial, Term};

pub trait Integration<T: Num + Pow<T, Output = T> + Copy> {
    /// Intergrates a copy of self, and returns the copy
    fn integrate_self(&self) -> Self;
    /// Returns self, intergrated to the upper and lower bound
    fn integrate(&self, lower: T, upper: T) -> T;
}

impl<T> Integration<T> for Polynomial<T>
where
    T: Num + Pow<T, Output = T> + Copy,
{
    fn integrate_self(&self) -> Self {
        let mut all = Vec::with_capacity(self.0.len());
        for x in self.0.iter() {
            all.push(x.integrate_self())
        }
        Polynomial(all)
    }

    fn integrate(&self, lower: T, upper: T) -> T {
        let mut total = zero();
        for x in self.0.iter() {
            total = total + x.sum_with_respect_to(&upper) - x.sum_with_respect_to(&lower);
        }
        total
    }
}

impl<T> Integration<T> for Term<T>
where
    T: Num + Pow<T, Output = T> + Copy,
{
    fn integrate_self(&self) -> Self {
        let exponent = self.exponent + one();
        Self::new(self.coefficient / exponent, exponent)
    }

    fn integrate(&self, upper: T, lower: T) -> T {
        let intergrated = self.integrate_self();
        intergrated.sum_with_respect_to(&upper) - intergrated.sum_with_respect_to(&lower)
    }
}
