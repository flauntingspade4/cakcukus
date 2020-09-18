use cakcukus::{terms, Term};

#[test]
fn terms() {
    let terms = terms!(2., 2., -3., 1.);

    assert_eq!(terms, vec![Term::new(2., 2.), Term::new(-3., 1.)]);
}

#[test]
fn term() {
    let term: Term<u32> = terms!(5, 2);

    assert_eq!(term, Term::new(5, 2))
}
