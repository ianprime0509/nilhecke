#[macro_use]
extern crate error_chain;

extern crate nilhecke;

use std::collections::HashSet;
use std::io::{self, Write};

use nilhecke::errors::*;
use nilhecke::{OddMonomial, OddPolynomial};

const VERSION: &str = "0.1.0";

fn main() {
    println!("This is NILHECKE version {}.", VERSION);
    loop {
        match run() {
            Ok(()) => break,
            Err(e) => {
                let stderr = &mut io::stderr();
                let errmsg = "Error writing to stderr";

                writeln!(stderr, "error: {}", e).expect(errmsg);

                for e in e.iter().skip(1) {
                    writeln!(stderr, "caused by: {}", e).expect(errmsg);
                }

                if let Some(backtrace) = e.backtrace() {
                    writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
                }
            }
        }
    }
}

fn run() -> Result<()> {
    loop {
        println!();
        match prompt("function:").as_str() {
            "print" => print()?,
            "add" => add()?,
            "mul" => mul()?,
            "p" => p()?,
            "schud" => schud()?,
            "" | "quit" | "bye" => break,
            _ => println!("unknown function"),
        }
    }

    println!("Bye!");
    Ok(())
}

fn print() -> Result<()> {
    println!("{}", prompt("polynomial:").parse::<OddPolynomial>()?);
    Ok(())
}

fn add() -> Result<()> {
    let p1 = prompt("p1:").parse::<OddPolynomial>()?;
    let p2 = prompt("p2:").parse::<OddPolynomial>()?;
    println!("{} + {} = {}", p1, p2, &p1 + &p2);

    Ok(())
}

fn mul() -> Result<()> {
    let p1 = prompt("p1:").parse::<OddPolynomial>()?;
    let p2 = prompt("p2:").parse::<OddPolynomial>()?;
    println!("{} * {} = {}", p1, p2, &p1 * &p2);

    Ok(())
}

fn p() -> Result<()> {
    let ops = prompt("operators:");
    let mut poly = prompt("poly:").parse::<OddPolynomial>()?;

    for op in ops.split_whitespace().rev() {
        let op_num = op[1..]
            .parse::<u32>()
            .chain_err(|| "invalid operator number")?;
        match op.chars().nth(0).unwrap() {
            's' => {
                if op_num >= 1 {
                    poly = poly.ps(op_num)
                } else {
                    bail!("s{} is not a valid operator", op_num)
                }
            }
            'b' => {
                if op_num >= 1 {
                    poly = poly.pb(op_num)
                } else {
                    bail!("b{} is not a valid operator", op_num)
                }
            }
            'd' => {
                if op_num >= 2 {
                    poly = poly.pd(op_num)
                } else {
                    bail!("d{} is not a valid operator")
                }
            }
            c => bail!("unknown operator symbol {}", c),
        }
    }

    println!("result: {}", poly);

    Ok(())
}

fn schud() -> Result<()> {
    let n = prompt("n:").parse::<u32>().chain_err(|| "invalid number")?;
    if n < 2 {
        bail!("invalid value for n");
    }
    let deltad = OddMonomial::deltad(n);
    let degree = deltad.degree();
    let mut schuberts = HashSet::new();
    schuberts.insert(OddPolynomial::from_monomial(deltad));

    for _ in 0..degree {
        let mut new = HashSet::new();
        for poly in &schuberts {
            for n in 1..n {
                new.insert(poly.ps(n));
            }
            new.insert(poly.pd(n));
        }
        for poly in new {
            schuberts.insert(poly);
        }
    }

    println!("schubert polynomials:");
    for poly in schuberts {
        println!("{}", poly);
    }

    Ok(())
}

fn prompt(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().expect("could not flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("could not read from stdin");
    input.trim().into()
}