use core::fmt::{Debug, Display};

use num_traits::{Num, Pow};

/// A macro to generate terms quickly. Can either be used to
/// generate a Vec of terms, or a single term
///
/// # Examples
/// ```
/// use cakcukus::{terms, prelude::*};
///
/// let terms = terms!(2., 2., -3., 1.); // Can generate a Vec of Terms,
///
/// assert_eq!(terms, vec![Term::new(2., 2.), Term::new(-3., 1.)]);
///
/// let term: Term<u32> = terms!(5, 2); // Or a single term
///
/// assert_eq!(term, Term::new(5, 2))
/// ```
#[macro_export]
macro_rules! terms {
    ($coefficient:expr, $exponent:expr) => {
        Term::new($coefficient, $exponent)
    };
    ($($coefficient:expr, $exponent:expr),*) => {{
        let mut vec = Vec::new();
        $( vec.push(Term::new($coefficient, $exponent)); )*
        vec
    }};
}

/// A Term is used to represent a single term in an equation.
///
/// It's worth pointing out that despite Term being generic,
/// signed intergers are not supported, as raising a number
/// to a negative power will return a floating point number,
/// which is not an integer. (This may be fixed in a future
/// update, but for now will be ignored.)
#[derive(Clone, Copy, PartialEq)]
pub struct Term<T>
where
    T: Num + Pow<T, Output = T> + From<u8> + Copy,
{
    /// The number that x is multiplied by in the term
    pub coefficient: T,

    /// The number that x is raised by in the term
    pub exponent: T,
}

impl<T: Num + Pow<T, Output = T> + From<u8> + Copy> Term<T> {
    /// Constructs a new term based off a given coefficient
    /// and exponent
    pub fn new(coefficient: T, exponent: T) -> Self {
        Self {
            coefficient,
            exponent,
        }
    }
}

impl<T: Num + Pow<T, Output = T> + From<u8> + Copy + Debug> Debug for Term<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}x^{:?}", self.coefficient, self.exponent)
    }
}

impl<T: Num + Pow<T, Output = T> + From<u8> + Copy + Display> Display for Term<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x^{}", self.coefficient, self.exponent)
    }
}
