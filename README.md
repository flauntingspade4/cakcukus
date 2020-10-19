![Cakcukus Banner](assets/banner.png)

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/flauntingspade4/cakcukus/blob/master/LICENSE)
[![Build](https://img.shields.io/github/workflow/status/flauntingspade4/cakcukus/Rust)]()
[![Issues](https://img.shields.io/github/issues/flauntingspade4/cakcukus)](https://github.com/flauntingspade4/cakcukus/issues)
[![PRs](https://img.shields.io/github/issues-pr/flauntingspade4/cakcukus)](https://github.com/flauntingspade4/cakcukus/pulls)

## About The Project

A very basic library for simple calculus, with a `Term` type, and three calulus traits

* `Integration` - A type's ability to perform basic integration
* `Differentiation` - A type's ability to perform basic differentiation
* `TermTrait` - A type's ability to be treated as a term in a polynomial

The built-in `Term` implements all three traits, but it's allowed for other types to implement them for similar functionality.
`Term` is also generic, but they type used for it's coefficient and exponent must have basic maths methods avaliable for them

## Usage

In Cargo.toml:

``` toml
[dependencies]
cakcukus = { git = "https://github.com/flauntingspade4/cakcukus" }
```

In main.rs:

``` rust
use cakcukus::{
    traits::{Differentiation, TermTrait},
    Term,
};

fn main() {
    // Build the initial equation, being 2x^2 - 3x + 5
    let terms = vec![Term::new(2., 2.), Term::new(-3., 1.), Term::new(5., 0.)];

    // This differentiates into 4x - 3
    let differentiated = terms.differentiate_self();

    // At x = 7, the gradient should be 25
    println!("{}", differentiated.sum_with_respect_to(&7.));

    // Outputs 25
}
```

## Roadmap

See the [open issues](https://github.com/flauntingspade4/cakcukus/issues) for a list of proposed features (and known issues).
