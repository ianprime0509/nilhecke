#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain!{
        errors {
            /// Invalid polynomial expression.
            ParsePolynomial(s: String) {
                description("invalid polynomial expression")
                display("invalid polynomial expression: {}", s)
            }
        }
    }
}

mod polynomial;

pub use polynomial::{OddPolynomial, OddMonomial};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
