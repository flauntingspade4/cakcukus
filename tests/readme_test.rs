#[cfg(feature = "built_in_types")]
use cakcukus::{
    terms,
    traits::{Differentiation, TermTrait},
    Polynomial, Term,
};

#[cfg(feature = "built_in_types")]
#[test]
fn test_vec() {
    // Build the initial equation, being 2x^2 - 3x + 5
    let terms: Polynomial<f64> = Polynomial(vec![
        Term::new(2., 2.),
        Term::new(-3., 1.),
        Term::new(5., 0.),
    ]);
    // Could also use `let terms = terms!(2., 2., -3., 1., 5., 0.);`

    // This differentiates into 4x - 3
    let differentiated = terms.differentiate_self();

    // At x = 7, the gradient should be 25
    assert!((25. - differentiated.sum_with_respect_to(&7.)).abs() < f64::EPSILON);

    // Outputs 25
}

#[cfg(feature = "built_in_types")]
#[test]
fn test_terms() {
    // Build the initial equation, being 2x^2 - 3x + 5
    let terms: Polynomial<f64> = terms![2., 2., -3., 1., 5., 0.];
    // Could also use:
    // Polynomial(vec![
    //     Term::new(2., 2.),
    //     Term::new(-3., 1.),
    //     Term::new(5., 0.),
    // ]);

    // This differentiates into 4x - 3
    let differentiated = terms.differentiate_self();

    // At x = 7, the gradient should be 25
    assert!((25. - differentiated.sum_with_respect_to(&7.)).abs() < f64::EPSILON);

    // Outputs 25
}
