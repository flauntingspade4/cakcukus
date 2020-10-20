//! A crate for basic, generic calculus functions.
//! The crate is build around the type [Term](term/struct.Term.html),
//! which is used to represent some `Coefficient * x ^ Exponent`.

mod polynomial;
pub mod term;
pub mod traits;

pub use polynomial::Polynomial;
pub use term::Term;
pub use traits::Calculus;
