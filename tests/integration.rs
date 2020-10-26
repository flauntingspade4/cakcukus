#[cfg(feature = "built_in_types")]
use cakcukus::{
    traits::{Integration, TermTrait},
    Polynomial, Term,
};

#[cfg(feature = "built_in_types")]
#[test]
fn intergrate_return() {
    // Builds the initial equaton, being x^4 + 2x + 4
    let terms = Polynomial(vec![
        Term::new(1., 4.),
        Term::new(2., 1.),
        Term::new(4., 0.),
    ]);

    // This then intergrates to 0.2x^5 + x^2 + 4x
    let intergrated = terms.integrate_self();

    assert!(18.4 - intergrated.sum_between(0., 2.) < f32::EPSILON);
}
