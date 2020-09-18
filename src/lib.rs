//! A crate for basic, generic calculus functions.
//! The crate is build around the type [Term](term/struct.Term.html),
//! which is used to represent some `Coefficient * x ^ Exponent`.
//!
//! The trait [Calculus](calculus/trait.calculus.html)
//! is used for the basic calculus methods.

pub mod calculus;
pub mod term;

pub use crate::calculus::Calculus;
pub use crate::term::Term;
