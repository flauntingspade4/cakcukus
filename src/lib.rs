//! A crate for basic, generic calculus functions.
//! The crate is build around the type [Term](term/struct.Term.html),
//! which is used to represent some `Coefficient * x ^ Exponent`.

pub mod term;
pub mod traits;

pub use crate::term::Term;
pub use crate::traits::Calculus;
