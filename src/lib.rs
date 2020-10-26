//! A crate for basic, generic calculus functions.
//! The crate is build around the type [Term](term/struct.Term.html),
//! which is used to represent some `Coefficient * x ^ Exponent`.

#[cfg(feature = "built_in_types")]
mod polynomial;
#[cfg(feature = "built_in_types")]
pub mod term;

pub mod traits;

#[cfg(feature = "built_in_types")]
pub use polynomial::Polynomial;
#[cfg(feature = "built_in_types")]
pub use term::Term;

pub use traits::Calculus;
