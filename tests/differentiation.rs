#[cfg(feature = "built_in_types")]
use cakcukus::{
    traits::{Differentiation, TermTrait},
    Polynomial, Term,
};

#[cfg(feature = "built_in_types")]
#[test]
fn differentiate_return() {
    // Build the initial equation, being 2x^2 - 3x + 5
    let terms: Polynomial<f32> = Polynomial(vec![
        Term::new(2., 2.),
        Term::new(-3., 1.),
        Term::new(5., 0.),
    ]);

    // This differentiates into 4x - 3
    let differentiated = terms.differentiate_self();

    // At x = 7, the gradient should be 25
    assert!(25. - differentiated.sum_with_respect_to(&7.) < f32::EPSILON);
}
