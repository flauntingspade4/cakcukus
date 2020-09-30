use cakcukus::{
    traits::{Integration, TermTrait},
    Term,
};

#[test]
fn intergrate_return() {
    // Builds the initial equaton, being x^4 + 2x + 4
    let terms = vec![Term::new(1., 4.), Term::new(2., 1.), Term::new(4., 0.)];

    // This then intergrates to 0.2x^5 + x^2 + 4x
    let intergrated = terms.intergrate_self();

    assert_eq!(18.4, intergrated.sum_between(0., 2.));
}
