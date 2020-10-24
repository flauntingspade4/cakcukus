use std::fmt::Debug;

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
        let sw = simple_stopwatch::Stopwatch::start_new();

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
                sorted.push(Term::new(current_coefficient, prev_exponent));
                current_coefficient = next_term.coefficient.clone();
                prev_exponent = next_term.exponent.clone();
            }
        }
        sorted.push(Term::new(current_coefficient, prev_exponent));

        self.0 = sorted;

        let ns = sw.ns();
        println!("Time taken to simplify: {}ns", ns,);
    }
}

mod impl_std_traits {
    use core::{
        fmt::Display,
        ops::{Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    };
    use std::ops::Neg;

    use crate::{Polynomial, Term};

    use num_traits::{Num, Pow};
    impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> Div for Polynomial<T> {
        type Output = Self;

        fn div(mut self, mut rhs: Self) -> Self::Output {
            println!("Simplifying `Self` in div");
            self.simplify();
            println!("Simplifying `rhs` in div");
            rhs.simplify();
            let mut to_return = Vec::new();
            for i in 0..self.0.len() {
                to_return.push(Term::new(
                    self.0[i].coefficient.clone(),
                    self.0[i].exponent.clone() - rhs.0[0].exponent.clone(),
                ));
                // self.0
            }
            for term in self.0.iter_mut() {
                to_return.push(term.clone() / rhs.0[0].clone());
            }
            /*for term in self.0.iter() {
                for rhs_term in rhs.0.iter() {
                    to_return.push(*term / *rhs_term);
                }
            }*/
            Self(to_return)
        }
    }
    // TODO Make this work
    /*impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> DivAssign for Polynomial<T> {
        fn div_assign(&mut self, rhs: Self) {
            let mut to_set = Vec::with_capacity(self.0.len() * rhs.0.len());
            for term in self.0.iter() {
                for rhs_term in rhs.0.iter() {
                    to_set.push(*term / *rhs_term);
                }
            }
            *self = Self(to_set)
        }
    }*/
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
    // Allowing Polynomial<T> to multiply by Polynomial<T>, Term<T> and T
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

    impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> Sub for Polynomial<T>
    where
        Term<T>: Neg<Output = Term<T>>,
    {
        type Output = Self;

        fn sub(mut self, rhs: Self) -> Self::Output {
            for rhs_term in rhs.0 {
                self.0.push(-rhs_term);
            }
            return self;
        }
    }
    impl<T: Num + Pow<T, Output = T> + Clone + PartialOrd> SubAssign for Polynomial<T>
    where
        Term<T>: Neg<Output = Term<T>>,
    {
        fn sub_assign(&mut self, rhs: Self) {
            for rhs_term in rhs.0 {
                self.0.push(-rhs_term);
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
