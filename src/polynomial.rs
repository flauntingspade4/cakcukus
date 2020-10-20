use std::fmt::Debug;

use crate::Term;

use num_traits::{identities::zero, Num, Pow};

#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial<T>(pub Vec<Term<T>>)
where
    T: Num + Pow<T, Output = T> + Copy;

impl<T> Polynomial<T>
where
    T: Num + Pow<T, Output = T> + Copy,
{
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<T> Default for Polynomial<T>
where
    T: Num + Pow<T, Output = T> + Copy,
{
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Polynomial<T>
where
    T: Num + Pow<T, Output = T> + Copy + PartialEq + PartialOrd,
{
    pub fn simplify(&mut self) {
        let mut current_coefficient = zero();
        let mut sorted: Vec<Term<T>> = Vec::new();
        let mut prev = match self.0.first() {
            Some(t) => t.coefficient,
            None => return,
        };
        self.0
            .sort_by(|a, b| b.exponent.partial_cmp(&a.exponent).unwrap());

        for next_term in self.0.iter() {
            if next_term.exponent == prev {
                current_coefficient = current_coefficient + next_term.coefficient;
            } else {
                sorted.push(Term::new(current_coefficient, prev));
                current_coefficient = next_term.coefficient;
                prev = next_term.exponent;
            }
        }
    }
}

mod impl_std_traits {
    use core::{
        fmt::Display,
        ops::{Div, DivAssign, Mul, MulAssign},
    };

    use crate::{Polynomial, Term};

    use num_traits::{Num, Pow};
    impl<T: Num + Pow<T, Output = T> + Copy> Div for Polynomial<T> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            let to_return = Vec::with_capacity(self.0.len());
            for _term in self.0.iter() {}
            Self(to_return)
        }
    }
    impl<T: Num + Pow<T, Output = T> + Copy> DivAssign for Polynomial<T> {
        fn div_assign(&mut self, rhs: Self) {}
    }
    impl<T: Num + Pow<T, Output = T> + Copy> Div<Term<T>> for Polynomial<T> {
        type Output = Self;

        fn div(self, rhs: Term<T>) -> Self::Output {
            let mut to_return = Vec::with_capacity(self.0.len());
            for term in self.0.iter() {
                to_return.push(*term / rhs);
            }
            Self(to_return)
        }
    }
    impl<T: Num + Pow<T, Output = T> + Copy> DivAssign<Term<T>> for Polynomial<T> {
        fn div_assign(&mut self, rhs: Term<T>) {
            for term in self.0.iter_mut() {
                *term = *term / rhs;
            }
        }
    }
    impl<T: Num + Pow<T, Output = T> + Copy> Div<T> for Polynomial<T> {
        type Output = Self;

        fn div(self, rhs: T) -> Self::Output {
            let mut to_return = Vec::with_capacity(self.0.len());
            for term in self.0.iter() {
                to_return.push(*term / rhs);
            }
            Self(to_return)
        }
    }
    impl<T: Num + Pow<T, Output = T> + Copy> DivAssign<T> for Polynomial<T> {
        fn div_assign(&mut self, rhs: T) {
            for term in self.0.iter_mut() {
                *term = *term / rhs;
            }
        }
    }
    // Allowing Polynomial<T> to multiply by Polynomial<T>, Term<T> and T
    impl<T: Num + Pow<T, Output = T> + Copy + PartialOrd + std::ops::AddAssign> Mul for Polynomial<T> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut to_return = Vec::with_capacity(self.0.len() * rhs.0.len());
            for term in self.0.iter() {
                for rhs_term in rhs.0.iter() {
                    to_return.push(*term * *rhs_term);
                }
            }
            Self(to_return)
        }
    }
    impl<T: Num + Pow<T, Output = T> + Copy> MulAssign for Polynomial<T> {
        fn mul_assign(&mut self, rhs: Self) {
            for term in self.0.iter_mut() {
                for rhs_term in rhs.0.iter() {
                    *term = *term * *rhs_term;
                }
            }
        }
    }
    impl<T: Num + Pow<T, Output = T> + Copy> Mul<Term<T>> for Polynomial<T> {
        type Output = Self;

        fn mul(self, rhs: Term<T>) -> Self::Output {
            let mut to_return = Vec::with_capacity(self.0.len());
            for term in self.0.iter() {
                to_return.push(*term * rhs);
            }
            Self(to_return)
        }
    }
    impl<T: Num + Pow<T, Output = T> + Copy> MulAssign<Term<T>> for Polynomial<T> {
        fn mul_assign(&mut self, rhs: Term<T>) {
            for term in self.0.iter_mut() {
                *term = *term * rhs;
            }
        }
    }

    impl<T: Num + Pow<T, Output = T> + Copy> Mul<T> for Polynomial<T> {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            let mut to_return = Vec::with_capacity(self.0.len());
            for term in self.0.iter() {
                to_return.push(*term * rhs);
            }
            Self(to_return)
        }
    }
    impl<T: Num + Pow<T, Output = T> + Copy> MulAssign<T> for Polynomial<T> {
        fn mul_assign(&mut self, rhs: T) {
            for term in self.0.iter_mut() {
                *term = *term * rhs;
            }
        }
    }

    impl<T: Num + Pow<T, Output = T> + Copy + Display> Display for Polynomial<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for term in self.0.iter() {
                write!(f, "{}x^{} + ", term.coefficient, term.exponent)?
            }
            Ok(())
        }
    }
}
