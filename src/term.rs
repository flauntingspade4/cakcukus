use num_traits::{Num, Pow};

/// A macro to generate terms quickly. Can either be used to
/// generate a Vec of terms, or a single term
///
/// # Examples
/// ```
/// use cakcukus::{terms, Polynomial, Term};
///
/// let terms = terms!(2., 2., -3., 1.); // Can generate a Vec of Terms,
///
/// assert_eq!(terms, Polynomial(vec![Term::new(2., 2.), Term::new(-3., 1.)]));
///
/// let term: Term<u32> = terms!(5, 2); // Or a single term
///
/// assert_eq!(term, Term::new(5, 2));
///
/// let term: Term<u32> = terms!(5);
///
/// assert_eq!(term, Term::new(5, 0));
/// ```
#[macro_export]
macro_rules! terms {
    ($coefficient:expr) => {
        cakcukus::Term::new($coefficient, 0)
    };
    ($coefficient:expr, $exponent:expr) => {
        cakcukus::Term::new($coefficient, $exponent)
    };
    ($($coefficient:expr, $exponent:expr),*) => {{
        let mut vec = Vec::new();
        $( vec.push(cakcukus::Term::new($coefficient, $exponent)); )*
        cakcukus::Polynomial(vec)
    }};
}

/// A Term is used to represent a single term in an equation.
///
/// It's worth pointing out that despite Term being generic,
/// signed intergers are not supported, as raising a number
/// to a negative power will return a floating point number,
/// which is not an integer. (This may be fixed in a future
/// update, but for now will be ignored.)
///
/// Term<T> can be multipled and divided by Term<T>, or T
#[derive(Clone, Copy, PartialEq)]
pub struct Term<T>
where
    T: Num + Pow<T, Output = T> + Clone,
{
    /// The number that x is multiplied by in the term
    pub coefficient: T,

    /// The number that x is raised by in the term
    pub exponent: T,
}

impl<T: Num + Pow<T, Output = T> + Clone> Term<T> {
    /// Constructs a new term based off a given coefficient
    /// and exponent
    pub fn new(coefficient: T, exponent: T) -> Self {
        Self {
            coefficient,
            exponent,
        }
    }
}

// Implementing std traits for Term
mod impl_std_traits {
    use core::{
        fmt::{Debug, Display},
        ops::{Div, DivAssign, Mul, MulAssign, Neg},
    };

    use crate::Term;

    use num_traits::{Num, Pow};
    // Allowing Term<T> to divide by Term<T> and T
    impl<T: Num + Pow<T, Output = T> + Clone> Div for Term<T> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            Self::new(
                self.coefficient / rhs.coefficient,
                self.exponent - rhs.exponent,
            )
        }
    }
    impl<T: Num + Pow<T, Output = T> + Clone> DivAssign for Term<T> {
        fn div_assign(&mut self, rhs: Self) {
            self.coefficient = self.coefficient.clone() / rhs.coefficient;
            self.exponent = self.exponent.clone() - rhs.exponent;
        }
    }
    impl<T: Num + Pow<T, Output = T> + Clone> Div<T> for Term<T> {
        type Output = Self;

        fn div(self, rhs: T) -> Self::Output {
            Self::new(self.coefficient / rhs, self.exponent)
        }
    }
    impl<T: Num + Pow<T, Output = T> + Clone> DivAssign<T> for Term<T> {
        fn div_assign(&mut self, rhs: T) {
            self.coefficient = self.coefficient.clone() / rhs;
        }
    }
    // Allowing Term<T> to multiply by Term<T> and T
    impl<T: Num + Pow<T, Output = T> + Clone> Mul for Term<T> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(
                self.coefficient * rhs.coefficient,
                self.exponent + rhs.exponent,
            )
        }
    }
    impl<T: Num + Pow<T, Output = T> + Clone> MulAssign for Term<T> {
        fn mul_assign(&mut self, rhs: Self) {
            self.coefficient = self.coefficient.clone() * rhs.coefficient;
            self.exponent = self.exponent.clone() + rhs.exponent;
        }
    }
    impl<T: Num + Pow<T, Output = T> + Clone> Mul<T> for Term<T> {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            Self::new(self.coefficient * rhs, self.exponent)
        }
    }
    impl<T: Num + Pow<T, Output = T> + Clone> MulAssign<T> for Term<T> {
        fn mul_assign(&mut self, rhs: T) {
            self.coefficient = self.coefficient.clone() * rhs;
        }
    }

    impl<T: Num + Pow<T, Output = T> + Clone + Neg<Output = T>> Neg for Term<T> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(-self.coefficient, self.exponent)
        }
    }

    impl<T: Num + Pow<T, Output = T> + Clone + Debug> Debug for Term<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}x^{:?}", self.coefficient, self.exponent)
        }
    }

    impl<T: Num + Pow<T, Output = T> + Clone + Display> Display for Term<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}x^{}", self.coefficient, self.exponent)
        }
    }
}
