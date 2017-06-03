use std::cmp;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Mul};
use std::str::FromStr;

use errors::*;

/// An odd monomial.
#[derive(Clone,Debug,Hash)]
pub struct OddMonomial {
    /// The coefficient of the monomial.
    coefficient: i32,
    /// The powers of each variable, in ascending order.
    powers: Vec<u32>,
}

impl OddMonomial {
    pub fn new(coefficient: i32, powers: Vec<u32>) -> OddMonomial {
        OddMonomial {
            coefficient,
            powers,
        }
    }

    /// Return a single variable
    pub fn x(n: u32) -> OddMonomial {
        let mut powers = Vec::new();
        for _ in 0..n - 1 {
            powers.push(0);
        }
        powers.push(1);

        OddMonomial {
            coefficient: 1,
            powers,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.coefficient == 0
    }

    /// `\sso_n`
    pub fn ss(&self, n: u32) -> Self {
        let n = n as usize;
        let mut coefficient = self.coefficient;
        let mut powers = self.powers.clone();
        for _ in powers.len()..n + 1 {
            powers.push(0);
        }

        // Update coefficient
        if (powers[n - 1] + powers[n]) % 2 != 0 {
            coefficient *= -1;
        }
        // Swap variables
        powers.swap(n - 1, n);

        OddMonomial::new(coefficient, powers)
    }

    /// `\sbo_n`
    pub fn sb(&self, n: u32) -> Self {
        let n = n as usize;
        let mut coefficient = self.coefficient;
        let mut powers = self.powers.clone();
        for _ in powers.len()..n {
            powers.push(0);
        }

        if powers[n - 1] % 2 != 0 {
            coefficient *= -1;
        }

        OddMonomial::new(coefficient, powers)
    }

    /// `\sdo_n`
    pub fn sd(&self, n: u32) -> Self {
        let n = n as usize;
        let mut powers = self.powers.clone();
        for _ in powers.len()..n + 1 {
            powers.push(0);
        }

        // Swap variables
        powers.swap(n - 2, n - 1);

        OddMonomial::new(self.coefficient, powers)
    }

    /// `\pso_n`
    pub fn ps(&self, n: u32) -> OddPolynomial {
        // Find first nonzero power to apply Leibniz rule
        let mut g = self.clone();
        let pos = match self.powers.iter().position(|&p| p != 0) {
            Some(pos) => pos as u32,
            None => return OddPolynomial::new(),
        };
        g.powers[pos as usize] -= 1;

        let ps_pos = if pos == n || pos == n - 1 { 1 } else { 0 };
        let mut first_term = g.clone();
        first_term.coefficient *= ps_pos;

        &OddPolynomial::from_monomial(first_term) +
        &(&OddPolynomial::from_monomial(OddMonomial::x(pos + 1).ss(n)) * &g.ps(n))
    }

    /// `\pbo_n`
    pub fn pb(&self, n: u32) -> OddPolynomial {
        // Find first nonzero power to apply Leibniz rule
        let mut g = self.clone();
        let pos = match self.powers.iter().position(|&p| p != 0) {
            Some(pos) => pos as u32,
            None => return OddPolynomial::new(),
        };
        g.powers[pos as usize] -= 1;

        let ps_pos = if pos == n - 1 { 1 } else { 0 };
        let mut first_term = g.clone();
        first_term.coefficient *= ps_pos;

        &OddPolynomial::from_monomial(first_term) +
        &(&OddPolynomial::from_monomial(OddMonomial::x(pos + 1).sb(n)) * &g.pb(n))
    }

    /// `\pdo_n`
    pub fn pd(&self, n: u32) -> OddPolynomial {
        // Find first nonzero power to apply Leibniz rule
        let mut g = self.clone();
        let pos = match self.powers.iter().position(|&p| p != 0) {
            Some(pos) => pos as u32,
            None => return OddPolynomial::new(),
        };
        g.powers[pos as usize] -= 1;

        let ps_pos = if pos == n - 2 {
            1
        } else if pos == n - 1 {
            -1
        } else {
            0
        };
        let mut first_term = g.clone();
        first_term.coefficient *= ps_pos;

        &OddPolynomial::from_monomial(first_term) +
        &(&OddPolynomial::from_monomial(OddMonomial::x(pos + 1).sd(n)) * &g.pd(n))
    }

    fn fmt_no_sign(&self, f: &mut Formatter) -> FmtResult {
        if self.is_zero() {
            return write!(f, "0");
        }

        // Position of the last nonzero power
        let pos = match self.powers.iter().rposition(|&power| power != 0) {
            Some(pos) => pos,
            None => return write!(f, "{}", self.coefficient),
        };

        if self.coefficient != 1 && self.coefficient != -1 {
            if self.coefficient < 0 {
                write!(f, "{}", -self.coefficient)?;
            } else {
                write!(f, "{}", self.coefficient)?;
            }
        }

        for (i, &power) in self.powers.iter().enumerate().take(pos) {
            if power != 0 {
                write!(f, "x_{}^{} ", i + 1, power)?;
            }
        }

        write!(f, "x_{}^{}", pos + 1, self.powers[pos])?;

        Ok(())
    }
}

impl Display for OddMonomial {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // Not the most efficient way to do this
        if self.coefficient < 0 {
            write!(f, "-")?;
        }
        self.fmt_no_sign(f)
    }
}

impl<'a, 'b> Mul<&'a OddMonomial> for &'b OddMonomial {
    type Output = OddMonomial;

    fn mul(self, other: &'a OddMonomial) -> OddMonomial {
        // We need to multiply the monomials, keeping track of sign changes.
        let mut powers = self.powers.clone();
        let mut coefficient = self.coefficient * other.coefficient;
        // Make sure `powers` is long enough
        for _ in powers.len()..other.powers.len() {
            powers.push(0);
        }
        // The number of variables x_2 to x_n
        let mut n_variables: u32 = self.powers.iter().skip(1).sum();

        for (i, &power) in other.powers.iter().enumerate() {
            powers[i] += power;
            if n_variables % 2 != 0 && power % 2 != 0 {
                coefficient *= -1;
            }
            if n_variables > 0 {
                n_variables -= powers[i + 1];
            }
        }

        OddMonomial {
            powers,
            coefficient,
        }
    }
}

/// An odd polynomial.
#[derive(Clone,Debug,Hash)]
pub struct OddPolynomial {
    /// The terms of the polynomial.
    terms: Vec<OddMonomial>,
}

impl OddPolynomial {
    pub fn new() -> Self {
        OddPolynomial { terms: Vec::new() }
    }

    pub fn from_monomial(monomial: OddMonomial) -> Self {
        if monomial.is_zero() {
            OddPolynomial::new()
        } else {
            OddPolynomial { terms: vec![monomial] }
        }
    }

    /// `\pso_n`
    pub fn ps(&self, n: u32) -> Self {
        let mut res = OddPolynomial::new();
        for term in &self.terms {
            res = &res + &term.ps(n);
        }
        res
    }

    /// `\pbo_n`
    pub fn pb(&self, n: u32) -> Self {
        let mut res = OddPolynomial::new();
        for term in &self.terms {
            res = &res + &term.pb(n);
        }
        res
    }

    /// `\pdo_n`
    pub fn pd(&self, n: u32) -> Self {
        let mut res = OddPolynomial::new();
        for term in &self.terms {
            res = &res + &term.pd(n);
        }
        res
    }

    fn add_monomial(&mut self, other: &OddMonomial) {
        if other.is_zero() {
            return;
        }

        // Try to add to an existing term if possible
        let pos = self.terms
            .iter()
            .position(|term| {
                for i in 0..cmp::max(term.powers.len(), other.powers.len()) {
                    if term.powers.get(i).unwrap_or(&0) != other.powers.get(i).unwrap_or(&0) {
                        return false;
                    }
                }
                true
            });
        if let Some(pos) = pos {
            self.terms[pos].coefficient += other.coefficient;
            if self.terms[pos].is_zero() {
                self.terms.remove(pos);
            }
        } else {
            self.terms.push(other.clone());
        }
    }
}

impl Display for OddPolynomial {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.terms.is_empty() {
            return write!(f, "0");
        }

        write!(f, "{}", self.terms[0])?;
        for term in self.terms.iter().skip(1) {
            if term.coefficient < 0 {
                write!(f, " - ")?;
            } else {
                write!(f, " + ")?;
            }
            term.fmt_no_sign(f)?;
        }

        Ok(())
    }
}

impl FromStr for OddPolynomial {
    type Err = Error;

    fn from_str(input: &str) -> Result<OddPolynomial> {
        let mut poly = OddPolynomial::new();

        for term in input.split('/') {
            let mut components = term.split_whitespace();
            let coefficient: i32 =
                components
                    .next()
                    .expect("empty term")
                    .parse()
                    .chain_err(|| ErrorKind::ParsePolynomial("invalid coefficient".into()))?;

            let mut powers = Vec::new();
            for c in components {
                powers.push(c.parse()
                                .chain_err(|| {
                                               ErrorKind::ParsePolynomial("invalid power".into())
                                           })?);
            }

            poly.add_monomial(&OddMonomial::new(coefficient, powers));
        }

        Ok(poly)
    }
}

impl<'a, 'b> Add<&'a OddPolynomial> for &'b OddPolynomial {
    type Output = OddPolynomial;

    fn add(self, other: &'a OddPolynomial) -> OddPolynomial {
        let mut poly = self.clone();
        for term in &other.terms {
            poly.add_monomial(term);
        }
        poly
    }
}

impl<'a, 'b> Mul<&'a OddPolynomial> for &'b OddPolynomial {
    type Output = OddPolynomial;

    fn mul(self, other: &'a OddPolynomial) -> OddPolynomial {
        let mut poly = OddPolynomial::new();

        for term1 in &self.terms {
            for term2 in &other.terms {
                poly.add_monomial(&(term1 * term2));
            }
        }

        poly
    }
}