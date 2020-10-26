use cakcukus::{terms, Polynomial};

#[test]
fn mult_test_t() {
    let polynomial: Polynomial<f64> = terms!(15., 2., 2., 1., 5., 0.);

    let mut polynomial_2 = polynomial.clone() * 2.;

    assert_eq!(polynomial_2, terms!(30., 2., 4., 1., 10., 0.));

    polynomial_2 *= 0.5;

    assert_eq!(polynomial, polynomial_2);
}

#[test]
fn mult_test_term_t() {
    let polynomial: Polynomial<f64> = terms!(15., 2., 2., 1., 5., 0.);

    let mut polynomial_2 = polynomial.clone() * terms!(2., 2.);

    assert_eq!(polynomial_2, terms!(30., 4., 4., 3., 10., 2.));

    polynomial_2 *= terms!(0.5, -2.);

    assert_eq!(polynomial, polynomial_2);
}

#[test]
fn mult_test_polynomial_t() {
    let polynomial: Polynomial<f64> = terms!(15., 2., 2., 1., 5., 0.);

    let mut polynomial_2 = polynomial.clone() * terms!(2., 4., 3., 2., 5., 0.);
    polynomial_2.simplify();

    assert_eq!(
        polynomial_2,
        terms!(30., 6., 4., 5., 55., 4., 6., 3., 90., 2., 10., 1., 25., 0.)
    );

    polynomial_2 *= terms!(2., 3.);

    let mut polynomial = polynomial * terms!(2., 4., 3., 2., 5., 0.) * terms!(2., 3.);
    polynomial.simplify();

    assert_eq!(polynomial_2, polynomial);
}

#[test]
fn div_test_t() {
    let polynomial: Polynomial<f64> = terms!(30., 2., 4., 1., 10., 0.);

    let mut polynomial_2 = polynomial.clone() / 2.;

    assert_eq!(polynomial_2, terms!(15., 2., 2., 1., 5., 0.));

    polynomial_2 /= 0.5;

    assert_eq!(polynomial, polynomial_2);
}

#[test]
fn div_test_term_t() {
    let polynomial: Polynomial<f64> = terms!(30., 4., 4., 3., 10., 2.);

    let mut polynomial_2 = polynomial.clone() / terms!(2., 2.);

    assert_eq!(polynomial_2, terms!(15., 2., 2., 1., 5., 0.));

    polynomial_2 /= terms!(0.5, -2.);

    assert_eq!(polynomial, polynomial_2);
}

#[test]
fn div_test_polynomial_t() {
    let polynomial: Polynomial<f64> = terms!(1., 3., 3., 2., -10., 1., -24., 0.);

    let mut polynomial_2 = polynomial.clone() / terms!(1., 2., 6., 1., 8., 0.);
    polynomial_2.simplify();

    assert_eq!(polynomial_2, terms!(1., 1., -3., 0.));

    polynomial_2 *= terms!(2., 3.);

    let mut polynomial = polynomial / terms!(1., 2., 6., 1., 8., 0.) * terms!(2., 3.);
    polynomial.simplify();

    assert_eq!(polynomial_2, polynomial);
}

#[test]
#[ignore]
fn simplify_bench() {
    use simple_stopwatch::Stopwatch;

    let mut polynomial = terms!();

    for i in 0..500 {
        let i = i as f64;
        polynomial.0.push(terms!(i * 12., i / (-2.5)));
    }

    let mut sw = Stopwatch::start_new();

    polynomial *= terms!(2.15, 345., 6.2, 2., 4.9, 5., 2., 0.);

    let ms = sw.ms();
    println!(
        "Time taken to multiply: {}ms, now {} terms",
        ms,
        polynomial.0.len()
    );
    sw.restart();

    polynomial.simplify();

    let ms = sw.ms();
    println!(
        "Time taken to simplify: {}ms, now {} terms",
        ms,
        polynomial.0.len()
    );
}
