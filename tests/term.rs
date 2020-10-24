use cakcukus::{terms, Polynomial, Term};

#[test]
fn terms() {
    let terms = terms!(2., 2., -3., 1.);

    assert_eq!(
        terms,
        Polynomial(vec![Term::new(2., 2.), Term::new(-3., 1.)])
    );
}

#[test]
fn term() {
    let term: Term<u32> = terms!(5, 2);

    assert_eq!(term, Term::new(5, 2))
}

#[test]
fn mult() {
    let mut term: Term<u32> = terms!(2, 1);

    term *= 15;

    assert_eq!(term, Term::new(30, 1));

    assert_eq!(term * terms!(1, 3), Term::new(30, 4));
}

#[test]
fn div() {
    let mut term: Term<u32> = terms!(30, 4);

    term /= 15;

    assert_eq!(term, Term::new(2, 4));

    assert_eq!(term / terms!(1, 3), Term::new(2, 1));
}
