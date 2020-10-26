use crate::Term;

use num_traits::{identities::zero, Num, Pow};

#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial<T>(pub Vec<Term<T>>)
where
    T: Num + Pow<T, Output = T> + Clone + PartialOrd;

impl<T> Polynomial<T>
where
    T: Num + Pow<T, Output = T> + Clone + PartialOrd,
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
    T: Num + Pow<T, Output = T> + Clone + PartialOrd,
{
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Polynomial<T>
where
    T: Num + Pow<T, Output = T> + Clone + PartialOrd,
{
    pub fn simplify(&mut self) {
        if self.0.is_empty() {
            return;
        }
        self.0
            .sort_unstable_by(|a, b| b.exponent.partial_cmp(&a.exponent).unwrap());
        let mut prev_exponent = self.0.first().unwrap().exponent.clone();
        let mut current_coefficient = zero();
        let mut sorted: Vec<Term<T>> = Vec::new();

        for next_term in self.0.iter() {
            if next_term.exponent == prev_exponent {
                current_coefficient = current_coefficient + next_term.coefficient.clone();
            } else {
                if current_coefficient != zero() {
                    sorted.push(Term::new(current_coefficient, prev_exponent));
                }
                current_coefficient = next_term.coefficient.clone();
                prev_exponent = next_term.exponent.clone();
            }
        }
        sorted.push(Term::new(current_coefficient, prev_exponent));

        self.0 = sorted;
    }
}

mod impl_std_traits {
    use crate::{Polynomial, Term};
    use core::{fmt::Display, ops::Neg};

    use num_traits::{Num, Pow};
    mod div {
        use super::{Neg, Num, Polynomial, Pow, Term};
        use std::ops::{Div, DivAssign};
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd + Neg<Output = T>> Div for Polynomial<T> {
            type Output = Self;

            fn div(mut self, mut rhs: Self) -> Self::Output {
                self.simplify();
                rhs.simplify();
                let mut to_return = Vec::new();
                while self.0.len() != 1 {
                    let to_push = Term::new(
                        self.0[0].coefficient.clone(),
                        self.0[0].exponent.clone() - rhs.0[0].exponent.clone(),
                    );
                    to_return.push(to_push.clone());
                    self -= rhs.clone() * to_push;
                }
                Self(to_return)
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd + Neg<Output = T>> DivAssign
            for Polynomial<T>
        {
            fn div_assign(&mut self, mut rhs: Self) {
                self.simplify();
                rhs.simplify();
                let mut to_return = Vec::new();
                while self.0.len() != 1 {
                    let to_push = Term::new(
                        self.0[0].coefficient.clone(),
                        self.0[0].exponent.clone() - rhs.0[0].exponent.clone(),
                    );
                    to_return.push(to_push.clone());
                    *self -= rhs.clone() * to_push;
                }
                self.0 = to_return;
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> Div<Term<T>> for Polynomial<T> {
            type Output = Self;

            fn div(self, rhs: Term<T>) -> Self::Output {
                let mut to_return = Vec::with_capacity(self.0.len());
                for term in self.0.iter() {
                    to_return.push(term.clone() / rhs.clone());
                }
                Self(to_return)
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> DivAssign<Term<T>> for Polynomial<T> {
            fn div_assign(&mut self, rhs: Term<T>) {
                for term in self.0.iter_mut() {
                    *term = term.clone() / rhs.clone();
                }
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> Div<T> for Polynomial<T> {
            type Output = Self;

            fn div(self, rhs: T) -> Self::Output {
                let mut to_return = Vec::with_capacity(self.0.len());
                for term in self.0.iter() {
                    to_return.push(term.clone() / rhs.clone());
                }
                Self(to_return)
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> DivAssign<T> for Polynomial<T> {
            fn div_assign(&mut self, rhs: T) {
                for term in self.0.iter_mut() {
                    *term = term.clone() / rhs.clone();
                }
            }
        }
    }
    mod mul {
        use super::{Num, Polynomial, Pow, Term};
        use std::ops::{Mul, MulAssign};
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> Mul for Polynomial<T> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut to_return = Vec::with_capacity(self.0.len() * rhs.0.len());
                for term in self.0.iter() {
                    for rhs_term in rhs.0.iter() {
                        to_return.push(term.clone() * rhs_term.clone());
                    }
                }
                Self(to_return)
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> MulAssign for Polynomial<T> {
            fn mul_assign(&mut self, rhs: Self) {
                let mut to_set = Vec::with_capacity(self.0.len() * rhs.0.len());
                for term in self.0.iter() {
                    for rhs_term in rhs.0.iter() {
                        to_set.push(term.clone() * rhs_term.clone());
                    }
                }
                *self = Self(to_set)
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> Mul<Term<T>> for Polynomial<T> {
            type Output = Self;

            fn mul(self, rhs: Term<T>) -> Self::Output {
                let mut to_return = Vec::with_capacity(self.0.len());
                for term in self.0.iter() {
                    to_return.push(term.clone() * rhs.clone());
                }
                Self(to_return)
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> MulAssign<Term<T>> for Polynomial<T> {
            fn mul_assign(&mut self, rhs: Term<T>) {
                for term in self.0.iter_mut() {
                    *term = term.clone() * rhs.clone();
                }
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> Mul<T> for Polynomial<T> {
            type Output = Self;

            fn mul(self, rhs: T) -> Self::Output {
                let mut to_return = Vec::with_capacity(self.0.len());
                for term in self.0.iter() {
                    to_return.push(term.clone() * rhs.clone());
                }
                Self(to_return)
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> MulAssign<T> for Polynomial<T> {
            fn mul_assign(&mut self, rhs: T) {
                for term in self.0.iter_mut() {
                    *term = term.clone() * rhs.clone();
                }
            }
        }
    }
    mod sub {
        use super::{Neg, Num, Polynomial, Pow, Term};
        use std::ops::{Sub, SubAssign};

        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd + Neg<Output = T>> Sub for Polynomial<T> {
            type Output = Self;

            fn sub(mut self, rhs: Self) -> Self::Output {
                for rhs_term in rhs.0 {
                    self.0.push(-rhs_term);
                }
                self
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd + Neg<Output = T>> SubAssign
            for Polynomial<T>
        {
            fn sub_assign(&mut self, rhs: Self) {
                for rhs_term in rhs.0 {
                    self.0.push(-rhs_term);
                }
                self.simplify();
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd + Neg<Output = T>> Sub<Term<T>>
            for Polynomial<T>
        {
            type Output = Self;

            fn sub(mut self, rhs: Term<T>) -> Self::Output {
                self.0.push(-rhs);

                self.simplify();
                self
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd + Neg<Output = T>> SubAssign<Term<T>>
            for Polynomial<T>
        {
            fn sub_assign(&mut self, rhs: Term<T>) {
                self.0.push(-rhs);
                self.simplify();
            }
        }
    }
    mod add {
        use super::{Num, Polynomial, Pow, Term};
        use std::ops::{Add, AddAssign};
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> Add for Polynomial<T> {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self.0.reserve(rhs.0.len());
                for rhs_term in rhs.0 {
                    self.0.push(rhs_term);
                }
                self
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> AddAssign for Polynomial<T> {
            fn add_assign(&mut self, rhs: Self) {
                self.0.reserve(rhs.0.len());
                for rhs_term in rhs.0 {
                    self.0.push(rhs_term);
                }
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> Add<Term<T>> for Polynomial<T> {
            type Output = Self;

            fn add(mut self, rhs: Term<T>) -> Self::Output {
                self.0.reserve(1);
                self.0.push(rhs);
                self
            }
        }
        impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> AddAssign<Term<T>> for Polynomial<T> {
            fn add_assign(&mut self, rhs: Term<T>) {
                self.0.reserve(1);
                self.0.push(rhs);
            }
        }
    }
    impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd + Display> Display for Polynomial<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for term in self.0.iter() {
                write!(f, "{}x^{} + ", term.coefficient, term.exponent)?
            }
            Ok(())
        }
    }
}
